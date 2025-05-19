use futures_util::stream::StreamExt;
use futures_util::SinkExt;
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


    // TODO: For a hint, see the description of the task below.
    loop {
        tokio::select! {
            // Read user input and send to server
            line = stdin.next_line() => {
                match line {
                    Ok(Some(text)) => {
                        if !text.is_empty() {
                            ws_stream.send(Message::text(text)).await?;
                        }
                    }
                    Ok(None) => break, // EOF
                    Err(e) => {
                        eprintln!("Error reading stdin: {e}");
                        break;
                    }
                }
            }
            // Receive message from server and print
            msg = ws_stream.next() => {
                match msg {
                    Some(Ok(msg)) => {
                        if let Some(text) = msg.as_text() {
                            // Prepended "Joseph's Computer - From server: " to the received text
                            println!("Joseph's Computer - From server: {text}");
                        }
                    }
                    Some(Ok(msg)) if msg.is_close() => {
                        println!("Connection closed by server.");
                        break;
                    }
                    None => {
                        println!("Connection closed by server.");
                        break;
                    }
                    Some(Err(e)) => {
                        eprintln!("WebSocket error: {e}");
                        break;
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())

}