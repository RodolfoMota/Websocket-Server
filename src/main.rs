use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, WebSocketStream, tungstenite::Message};
use futures::{StreamExt, SinkExt};
use std::sync::{Arc, RwLock};

#[derive(Debug)]
struct Session {
    id: usize,
    addr: SocketAddr,
    sender: futures::channel::mpsc::UnboundedSender<Message>,
}

type Sessions = Arc<RwLock<Vec<Session>>>;

async fn handle_connection(sessions: Sessions, mut ws_stream: WebSocketStream<TcpStream>) {
    let (tx, mut rx) = futures::channel::mpsc::unbounded();
    let addr = ws_stream.get_ref().peer_addr().expect("Connected stream should have a peer address.");
    let id = sessions.read().unwrap().len();

    let session = Session {
        id,
        addr,
        sender: tx,
    };
    sessions.write().unwrap().push(session);


    let sessions_clone = sessions.clone();
    tokio::spawn(async move {
        while let Some(Message::Text(text)) = rx.next().await {
            for session in sessions_clone.read().unwrap().iter() {
                if let Err(e) = session.sender.unbounded_send(Message::Text(text.clone())) {
                    eprintln!("Error sending message to {}: {}", session.addr, e);
                }
            }
        }
    });

    while let Some(message) = ws_stream.next().await {
        match message {
            Ok(Message::Text(text)) => {
                println!("Received a text message from {}: {}", addr, text);
                ws_stream.send(Message::Text(text)).await.unwrap();
            }
            Ok(Message::Binary(bin)) => {
                println!("Received binary data from {}", addr);
                ws_stream.send(Message::Binary(bin)).await.unwrap();
            }
            Err(e) => {
                eprintln!("Error in connection with {}: {}", addr, e);
                break;
            }
            _ => {}
        }
    }

    // Remove the disconnected session from the shared sessions list
    sessions.write().unwrap().retain(|s| s.id != id);
    println!("{} disconnected", addr);
}



#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:9001".to_string();
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind address");
    println!("WebSocket server listening on {}", addr);

    let sessions: Sessions = Arc::new(RwLock::new(Vec::new()));

    while let Ok((stream, _)) = listener.accept().await {
        let ws_stream = accept_async(stream).await.expect("Failed to accept incoming connection");
        let sessions_clone = sessions.clone();
        tokio::spawn(handle_connection(sessions_clone, ws_stream));
    }
}
