use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::RwLock;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug)]
pub struct Node {
    pub address: String,
}

pub struct Network {
    nodes: Arc<RwLock<HashMap<String, Node>>>,
}

impl Network {
    pub fn new() -> Self {
        Network {
            nodes: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add_node(&self, node_id: String, node_address: String) {
        let mut nodes = self.nodes.write().await;
        nodes.insert(node_id, Node { address: node_address });
    }

    pub async fn start(&self, address: &str) {
        let listener = TcpListener::bind(address).await.unwrap();

        loop {
            let (socket, _) = listener.accept().await.unwrap();

            let nodes = self.nodes.clone();
            tokio::spawn(async move {
                handle_connection(socket, nodes).await;
            });
        }
    }
}

async fn handle_connection(socket: TcpStream, nodes: Arc<RwLock<HashMap<String, Node>>>) {
    let mut buffer = vec![0; 1024];
    let mut socket = socket;

    loop {
        let read_bytes = socket.read(&mut buffer).await.unwrap();

        if read_bytes == 0 {
            // Connection closed
            break;
        }

        let message = String::from_utf8_lossy(&buffer[..read_bytes]);

        // Process the message and perform actions based on its content
        // For example, forward the message to other nodes in the network

        let nodes = nodes.read().await;

        for node in nodes.values() {
            let mut node_socket = TcpStream::connect(&node.address).await.unwrap();
            node_socket.write_all(message.as_bytes()).await.unwrap();
        }
    }
}
