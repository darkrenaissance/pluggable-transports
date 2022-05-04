use anyhow::Result;
use async_trait::async_trait;
use futures::{AsyncRead, AsyncWrite};
use url::Url;

mod tcp;
pub use tcp::TcpTransport;

mod tor;
pub use tor::TorTransport;

mod tls;
pub use tls::TlsTransport;

pub trait AsyncRW: AsyncRead + AsyncWrite + Send + Unpin {}

#[async_trait]
pub trait Transport {
    type Output;

    async fn dial(self, url: Url) -> Result<Self::Output>
    where
        Self: Sized;

    async fn upgrade(
        self,
        stream: Self::Output,
        proto: &str,
    ) -> Result<Box<dyn Transport<Output = dyn AsyncRW>>>
    where
        Self: Sized;
}
