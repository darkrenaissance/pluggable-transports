use std::{io, net::SocketAddr};

use anyhow::Result;
use async_std::net::TcpStream;
use async_trait::async_trait;
use socket2::{Domain, Socket, Type};
use url::Url;

use super::{AsyncRW, TlsTransport, Transport};

pub struct TcpTransport {
    /// TTL to set for opened sockets, or `None` for default.
    ttl: Option<u32>,
    /// Size of the listen backlog for listen sockets
    backlog: u32,
}

impl AsyncRW for TcpStream {}

#[async_trait]
impl Transport for TcpTransport {
    type Output = TcpStream;

    async fn dial(self, url: Url) -> Result<Self::Output> {
        let socket_addr = url.socket_addrs(|| None)?[0];
        let socket = self.create_socket(socket_addr)?;
        socket.set_nonblocking(true)?;

        match socket.connect(&socket_addr.into()) {
            Ok(()) => {}
            Err(err) if err.raw_os_error() == Some(libc::EINPROGRESS) => {}
            Err(err) if err.kind() == io::ErrorKind::WouldBlock => {}
            Err(err) => return Err(err.into()),
        };

        let stream = TcpStream::from(std::net::TcpStream::from(socket));
        Ok(stream)
    }

    async fn upgrade(
        self,
        stream: Self::Output,
        proto: &str,
    ) -> Result<Box<dyn Transport<Output = dyn AsyncRW>>> {
        match proto {
            "tls" => Ok(Box::new(TlsTransport::new(stream))),
            _ => unimplemented!(),
        }
    }
}

impl TcpTransport {
    pub fn new() -> Self {
        Self { ttl: None, backlog: 1024 }
    }

    pub fn create_socket(&self, socket_addr: SocketAddr) -> Result<Socket> {
        let domain = if socket_addr.is_ipv4() { Domain::IPV4 } else { Domain::IPV6 };
        let socket = Socket::new(domain, Type::STREAM, Some(socket2::Protocol::TCP))?;

        if socket_addr.is_ipv6() {
            socket.set_only_v6(true)?;
        }

        if let Some(ttl) = self.ttl {
            socket.set_ttl(ttl)?;
        }

        Ok(socket)
    }
}
