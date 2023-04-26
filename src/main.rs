mod digital_id;
mod audit_trail;
mod network;

use digital_id::DigitalID;
use audit_trail::AuditTrail;
use network::Network;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let network = Arc::new(Network::new());

    // Add nodes to the network (replace with actual node IDs and addresses)
    network.add_node("node1".to_string(), "127.0.0.1:8081".to_string()).await;
    network.add_node("node2".to_string(), "127.0.0.1:8082".to_string()).await;

    let network_clone = network.clone();
    tokio::spawn(async move {
        network_clone.start("127.0.0.1:8080").await;
    });

    let digital_id = DigitalID::new("voter1".to_string());

    let id_verified = digital_id.verify_digital_id();
    println!("Digital ID verification: {}", id_verified);

    let mut audit_trail = AuditTrail::new("voter1".to_string(), "candidate1".to_string());

    audit_trail.verify_vote(&digital_id);
    println!("Vote verification: {}", audit_trail.verified);

    // Send the vote to other nodes in the network
    // (This example assumes the listener is running and other nodes are available)
    // let mut socket = TcpStream::connect("127.0.0.1:8081").await.unwrap();
    // let msg = format!("VOTER_ID {}\nVOTE {}\nSEND\n", audit_trail.user_id, audit_trail.vote);
    // socket.write_all(msg.as_bytes()).await.unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}
