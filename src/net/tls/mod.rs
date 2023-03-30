#[cfg(feature = "default-tls")]
mod native;

#[cfg(feature = "rustls-tls")]
mod rustls;

use anyhow::Result;
use std::net::TcpStream;

#[derive(Clone)]
pub enum TlsStream {
    #[cfg(feature = "default-tls")]
    NativeTLS(native::NativeTlsStream),
    #[cfg(feature = "rustls-tls")]
    Rustls(rustls::RustlsTlsStream),
}

#[cfg(all(feature = "default-tls", not(feature = "rustls-tls")))]
pub fn connect_tls(host: &str, verify_cert: bool, stream: TcpStream) -> Result<TlsStream> {
    native::tls_connect(host, verify_cert, stream)
}

#[cfg(all(feature = "rustls-tls", not(feature = "default-tls")))]
pub fn connect_tls(host: &str, verify_cert: bool, stream: TcpStream) -> Result<TlsStream> {
    rustls::tls_connect(host, verify_cert, stream)
}

#[cfg(not(any(feature = "rustls-tls", feature = "default-tls")))]
pub fn connect_tls(_host: &str, _verify_cert: bool, _stream: TcpStream) -> Result<TlsStream> {
    panic!("must enable one of default-tls or rustls-tls feature")
}
