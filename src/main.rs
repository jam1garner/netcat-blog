#[feature(async_await)]

use tokio::net::TcpListener;
use tokio::prelude::*;

mod server;
mod post_handler;
mod blog_data;

use server::SocketUtil;
use post_handler::PostHandler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        let mut socket = SocketUtil::new(socket);

        tokio::spawn(async move {
            let mut buf = [0; 1024];
            let mut post_handler = PostHandler::new();
            loop {
                if let Err(e) = socket.render(&post_handler).await {
                    match e.kind() {
                        std::io::ErrorKind::BrokenPipe => {}
                        _ => {
                            println!("failed to render; err = {:?}", e);
                        }
                    }
                }

                let ansi = match socket.read_ansi(&mut buf).await {
                    Ok(ansi_sequences) => ansi_sequences,
                    Err(e) => {
                        println!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                post_handler.handle_inputs(ansi);
            }
        });
    }
}
