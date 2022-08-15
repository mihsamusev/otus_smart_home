use smart_home::device::server::SmartSocketServer;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888")
        .await
        .expect("Could not bind to given address");
    let mut socket = SmartSocketServer::new(listener);
    socket.listen().await;
}
