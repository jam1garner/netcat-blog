///! Rendering for blog post stuff
use super::SocketUtil;
use crate::post_handler::PostHandler;

use ansi_parser::AnsiSequence as Seq;
use crate::server::ansi::Ansi;

static TEST_TEXT: &str = "Test 1\nTest 2\nTest 3";

pub async fn render(socket: &mut SocketUtil, posts: &PostHandler) -> Result<(), std::io::Error> {
    socket.write_ansi(Ansi::Escape(Seq::EraseDisplay)).await?;
    socket.write_ansi(Ansi::Escape(Seq::CursorPos(0,0))).await?;
    socket.write_ansi(Ansi::Escape(Seq::SetGraphicsMode(vec![4, 32]))).await?;
    socket.write_ansi(Ansi::Text("Title goes here\n".to_string())).await?;
    socket.write_ansi(Ansi::Escape(Seq::SetGraphicsMode(vec![0]))).await?;
    socket.write_ansi(Ansi::Text(
        TEST_TEXT.split("\n")
                 .skip(posts.scroll)
                 .map(String::from)
                 .collect::<Vec<String>>()
                 .join("\n")
    )).await?;
    Ok(())
}
