use tokio::prelude::*;
use tokio::net::tcp::TcpStream;

use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::post_handler::PostHandler;

pub(crate) mod ansi;
mod render;

pub struct SocketUtil {
    socket: TcpStream
}

impl SocketUtil {
    pub fn new(socket: TcpStream) -> Self {
        Self {
            socket
        }
    }

    pub async fn read_ansi(&mut self, buf: &mut [u8]) -> Result<Vec<ansi::Ansi>, std::io::Error> {
        let n = self.read(buf).await?;
        ansi::parse_bytes(&buf[..n])
    }

    pub async fn render(&mut self, handler: &PostHandler) -> Result<(), std::io::Error> {
        render::render(self, handler).await?;
        Ok(())
    }

    pub async fn write_ansi(&mut self, ansi_sequence: ansi::Ansi) -> Result<(), std::io::Error> {
        self.write(ansi::to_string(ansi_sequence).as_bytes()).await?;
        Ok(())
    }
}

impl AsyncRead for SocketUtil {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        TcpStream::poll_read(Pin::new(&mut self.get_mut().socket), cx, buf)
    }
}

impl AsyncWrite for SocketUtil {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        TcpStream::poll_write(Pin::new(&mut self.get_mut().socket), cx, buf)
    }

    #[inline]
    fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<io::Result<()>> {
        // tcp flush is a no-op
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.get_mut().socket.shutdown(std::net::Shutdown::Write)?;
        Poll::Ready(Ok(()))
    }
}
