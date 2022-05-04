use anyhow::Result;
use async_std::net::TcpStream;
use async_trait::async_trait;
use fast_socks5::client::Socks5Stream;
use url::Url;

use super::{AsyncRW, TlsTransport, Transport};

pub struct TorTransport {
    socks_url: Url,
}

impl TorTransport {
    pub fn new(socks_url: Url) -> Result<Self> {
        Ok(Self { socks_url })
    }
}

impl AsyncRW for Socks5Stream<TcpStream> {}

#[async_trait]
impl Transport for TorTransport {
    type Output = Socks5Stream<TcpStream>;

    async fn dial(self, url: Url) -> Result<Self::Output> {
        let socks_url_str = self.socks_url.socket_addrs(|| None)?[0].to_string();
        let host = url.host().unwrap().to_string();
        let port = url.port().unwrap();
        let config = fast_socks5::client::Config::default();

        let stream = Socks5Stream::connect(socks_url_str, host, port, config).await?;
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
