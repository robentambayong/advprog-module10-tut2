use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{Sender, channel};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};

async fn handle_connection(
    addr: SocketAddr,
    mut ws_stream: WebSocketStream<TcpStream>,
    bcast_tx: Sender<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    
    let mut bcast_rx = bcast_tx.subscribe();
    
    ws_stream.send(Message::text("Welcome to chat! Type a message")).await?;

    loop {
        tokio::select! {
            // Task 1: Receive messages from this client and broadcast them
            incoming = ws_stream.next() => {
                match incoming {
                    Some(Ok(msg)) if msg.is_text() => {
                        let text = msg.as_text().unwrap();
                        bcast_tx.send(text.to_string())?;
                    }
                    Some(Err(e)) => {
                        eprintln!("Error from {}: {}", addr, e);
                        break;
                    }
                    None => break, // Client disconnected
                    _ => {}
                }
            }
            // Task 2: Receive messages from the broadcast channel and send to this client
            msg = bcast_rx.recv() => {
                if let Ok(text) = msg {
                    ws_stream.send(Message::text(text)).await?;
                }
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (bcast_tx, _) = channel(16);

    let listener = TcpListener::bind("127.0.0.1:2000").await?;
    println!("listening on port 2000");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {:?}", addr);
        let bcast_tx = bcast_tx.clone();
        
        tokio::spawn(async move {
            match ServerBuilder::new().accept(socket).await {
                Ok((_req, ws_stream)) => {
                    if let Err(e) = handle_connection(addr, ws_stream, bcast_tx).await {
                        eprintln!("Error handling connection: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Error accepting websocket connection: {}", e);
                }
            }
        });
    }
}