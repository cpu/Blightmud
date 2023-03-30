use crate::net::RwStream;
use anyhow::Result;
use native_tls::TlsStream;
use std::net::TcpStream;

pub(crate) type NativeTlsStream = RwStream<TlsStream<TcpStream>>;

pub fn tls_connect(host: &str, verify_cert: bool, stream: TcpStream) -> Result<super::TlsStream> {
    let connector = native_tls::TlsConnector::builder()
        .danger_accept_invalid_certs(!verify_cert)
        .build()?;
    Ok(super::TlsStream::NativeTLS(RwStream::new(
        connector.connect(host, stream)?,
    )))
}
