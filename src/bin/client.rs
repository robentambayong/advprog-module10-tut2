use futures_util::SinkExt;
use futures_util::stream::StreamExt;
use http::Uri;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_websockets::{ClientBuilder, Message};

#[tokio::main]
async fn main() -> Result<(), tokio_websockets::Error> {
    let (mut ws_stream, _) =
        ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:8080"))
            .connect()
            .await?;

    let stdin = tokio::io::stdin();
    let mut stdin = BufReader::new(stdin).lines();

    loop {
        tokio::select! {
            // Task 1: Receive messages from the server
            incoming = ws_stream.next() => {
                match incoming {
                    Some(Ok(msg)) if msg.is_text() => {
                        println!("Roben's Komputer From server: {}", msg.as_text().unwrap());
                    }
                    Some(Err(e)) => {
                        eprintln!("Error receiving from server: {}", e);
                        break;
                    }
                    None => break,
                    _ => {}
                }
            }
            // Task 2: Read input from standard input and send to server
            line = stdin.next_line() => {
                match line {
                    Ok(Some(text)) => {
                        ws_stream.send(Message::text(text)).await?;
                    }
                    Ok(None) => break, // EOF
                    Err(e) => {
                        eprintln!("Error reading from stdin: {}", e);
                        break;
                    }
                }
            }
        }
    }
    Ok(())
}