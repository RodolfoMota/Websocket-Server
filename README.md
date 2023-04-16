# Websocket Server
This is a simple WebSocket server written in Rust that allows clients to connect, join or create groups, and exchange text and binary messages. The server uses the tokio and tokio_tungstenite libraries to implement asynchronous I/O and WebSocket functionality.

The server maintains a list of Session objects, each representing a connected client. Each Session object has a unique ID, a socket address, and a channel for sending messages to other connected clients.

The server also maintains a list of Group objects, each representing a group of connected clients. Each Group object has a unique code and a list of session IDs for the clients in the group.

When a client connects to the server, a new Session object is created and added to the shared list of sessions. The client can then join or create a group by sending commands over the WebSocket connection. Clients in the same group can exchange messages with each other by sending messages over their respective Session channels.
