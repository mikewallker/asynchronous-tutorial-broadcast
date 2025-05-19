# experiment 2.1
![alt text](image.png)
To run the chat application:

Start the server
Open a terminal and run:

cargo run --bin server


This will start the server and listen for WebSocket connections on ws://127.0.0.1:2000.

Start one or more clients
In separate terminals, run:

cargo run --bin client


When I type a message in one client and press Enter, the message is sent to the server.
The server broadcasts this message to all connected clients (including the sender).
Every client displays the message in their terminal, prefixed by the sender's address (e.g., 127.0.0.1:52093: halo).
This means all clients see every message sent by any client, including their own.
It's a simple real-time chat: every message I type is instantly visible to all connected clients.

# experiment 2.2
Yes, the server code is using the same WebSocket protocol as the client.

Both the server (server.rs) and client (client.rs) use the tokio_websockets crate, which implements the WebSocket protocol.
On the server side, the line:

let (_req, ws_stream) = ServerBuilder::new().accept(socket).await?;

upgrades the incoming TCP connection to a WebSocket connection.
On the client side, the line:

let (mut ws_stream, _) = ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:8080")).connect().await?;

connects to the server using the WebSocket protocol.

Both sides use the WebSocket protocol for communication, ensuring compatibility and real-time, bidirectional messaging.