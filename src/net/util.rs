use std::{
    net::{TcpStream, ToSocketAddrs},
    time::Duration,
};

use anyhow::{bail, Result};
use socket2::{Domain, Socket, TcpKeepalive, Type};

pub fn open_tcp_stream(host: &str, port: u16) -> Result<TcpStream> {
    let mut addr_iter = (host, port).to_socket_addrs()?;

    let timeout = Duration::new(3, 0);
    let stream = addr_iter.find_map(|addr| {
        let socket = Socket::new(Domain::IPV4, Type::STREAM, None).ok()?;
        socket.connect_timeout(&addr.into(), timeout).ok();
        socket
            .set_tcp_keepalive(
                &TcpKeepalive::new()
                    // After how long should keep-alives be sent on an idle connection?
                    .with_time(Duration::from_secs(5))
                    // And how long should we wait between keep-alives?
                    .with_interval(Duration::from_secs(5))
                    // How many keep-alives to send before considering the connection dead?
                    .with_retries(5),
            )
            .ok();
        Some(socket)
    });

    if let Some(stream) = stream {
        Ok(stream.into())
    } else {
        bail!("Invalid connection params")
    }
}
