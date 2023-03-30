use crate::net::RwStream;
use anyhow::Result;
use rustls::{ClientConfig, ClientConnection, OwnedTrustAnchor, RootCertStore, StreamOwned};
use std::net::TcpStream;
use std::sync::Arc;

pub(crate) type RustlsTlsStream = RwStream<StreamOwned<ClientConnection, TcpStream>>;

mod danger {
    use rustls::{client, Certificate, ServerName};

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

pub fn tls_connect(host: &str, verify_cert: bool, stream: TcpStream) -> Result<super::TlsStream> {
    let mut root_store = RootCertStore::empty();
    root_store.add_server_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.0.iter().map(|ta| {
        OwnedTrustAnchor::from_subject_spki_name_constraints(
            ta.subject,
            ta.spki,
            ta.name_constraints,
        )
    }));

    let mut config = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    if !verify_cert {
        config
            .dangerous()
            .set_certificate_verifier(Arc::new(danger::NoCertificateVerification {}));
    };

    let server_name = host.try_into()?;
    let conn = rustls::ClientConnection::new(Arc::new(config), server_name)?;
    Ok(super::TlsStream::Rustls(RwStream::new(
        rustls::StreamOwned::new(conn, stream),
    )))
}
