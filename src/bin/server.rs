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

    // Send a welcome message to the newly connected client
    if ws_stream.send(Message::text("Welcome to chat! Type a message".to_string())).await.is_err() {
        // Error sending welcome message, client might have disconnected
        return Ok(());
    }

    loop {
        tokio::select! {
            // Receive a message from the websocket client
            msg = ws_stream.next() => {
                match msg {
                    Some(Ok(msg)) => {
                        if msg.is_text() {
                            let text = msg.as_text().unwrap_or_default().to_string();
                            // Log received message on server
                            println!("From client {} \"{}\"", addr, text);
                            // Format message for broadcasting
                            let msg_to_broadcast = format!("{addr}: {text}");
                            let _ = bcast_tx.send(msg_to_broadcast);
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

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("listening on port 8080");

    loop {
        let (socket, addr) = listener.accept().await?;
        // Updated new connection log
        println!("New connection from Joseph's Computer{addr}");
        let bcast_tx = bcast_tx.clone();
        tokio::spawn(async move {
            // Wrap the raw TCP stream into a websocket.
            let (_req, ws_stream) = ServerBuilder::new().accept(socket).await?;

            handle_connection(addr, ws_stream, bcast_tx).await
        });
    }
}