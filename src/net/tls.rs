use crate::net::RwStream;
use anyhow::Result;
use rustls::{ClientConfig, ClientConnection, OwnedTrustAnchor, RootCertStore, StreamOwned};
use std::net::TcpStream;
use std::sync::Arc;

/// TlsStream is an alias for a read/write stream over an owned TLS client connection stream
/// using a TCP transport.
pub(super) type TlsStream = RwStream<StreamOwned<ClientConnection, TcpStream>>;

impl TlsStream {
    /// new constructs a [TlsStream] by attempting to establish a TLS session over the given
    /// [TcpStream] for the provided hostname.
    ///
    /// ## DANGER
    /// If the `verify_cert` bool is set to false no certificate verification is performed and
    /// the connection is vulnerable to person-in-the-middle attacks and tampering.
    pub(super) fn tls_init(stream: TcpStream, host: &str, verify_cert: bool) -> Result<TlsStream> {
        let mut config = ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(Self::root_certs())
            .with_no_client_auth();

        if !verify_cert {
            config
                .dangerous()
                .set_certificate_verifier(Arc::new(danger::NoCertificateVerification {}));
        };

        let server_name = host.try_into()?;
        let conn = ClientConnection::new(Arc::new(config), server_name)?;
        Ok(RwStream::new(StreamOwned::new(conn, stream)))
    }

    fn root_certs() -> RootCertStore {
        let mut root_store = RootCertStore::empty();
        root_store.add_server_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.0.iter().map(|ta| {
            OwnedTrustAnchor::from_subject_spki_name_constraints(
                ta.subject,
                ta.spki,
                ta.name_constraints,
            )
        }));
        root_store
    }
}

/// here be dragons.
mod danger {
    use rustls::{client, Certificate, ServerName};

    /// NoCertificateVerification is a **DANGEROUS** [client::ServerCertVerifier] that
    /// performs **no** certificate validation.
    pub struct NoCertificateVerification {}

    impl client::ServerCertVerifier for NoCertificateVerification {
        fn verify_server_cert(
            &self,
            _end_entity: &Certificate,
            _intermediates: &[Certificate],
            _server_name: &ServerName,
            _scts: &mut dyn Iterator<Item = &[u8]>,
            _ocsp: &[u8],
            _now: std::time::SystemTime,
        ) -> Result<client::ServerCertVerified, rustls::Error> {
            Ok(client::ServerCertVerified::assertion())
        }
    }
}
