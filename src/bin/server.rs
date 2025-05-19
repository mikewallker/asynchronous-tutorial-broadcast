use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{channel, Sender};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};

async fn handle_connection(
    addr: SocketAddr,
    mut ws_stream: WebSocketStream<TcpStream>,
    bcast_tx: Sender<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Subscribe to the broadcast channel
    let mut bcast_rx = bcast_tx.subscribe();

    loop {
        tokio::select! {
            // Receive a message from the websocket client
            msg = ws_stream.next() => {
                match msg {
                    Some(Ok(msg)) => {
                        if msg.is_text() {
                            let text = msg.as_text().unwrap_or_default().to_string();
                            // Broadcast the message to all clients
                            let msg = format!("{addr}: {text}");
                            let _ = bcast_tx.send(msg);
                        } else if msg.is_close() {
                            // Client disconnected
                            break;
                        }
                    }
                    Some(Err(e)) => {
                        eprintln!("WebSocket error from {addr}: {e}");
                        break;
                    }
                    None => {
                        // Client disconnected
                        break;
                    }
                }
            }
            // Receive a message from the broadcast channel
            Ok(broadcast_msg) = bcast_rx.recv() => {
                // Optionally, skip sending to the sender
                // if !broadcast_msg.starts_with(&format!("{addr}:")) {
                ws_stream.send(Message::text(broadcast_msg)).await?;
                // }
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
        println!("New connection from {addr:?}");
        let bcast_tx = bcast_tx.clone();
        tokio::spawn(async move {
            // Wrap the raw TCP stream into a websocket.
            let (_req, ws_stream) = ServerBuilder::new().accept(socket).await?;

            handle_connection(addr, ws_stream, bcast_tx).await
        });
    }
}