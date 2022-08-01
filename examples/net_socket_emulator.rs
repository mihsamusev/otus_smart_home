use smart_home::device::server::SmartSocketServer;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").expect("Could not bind to given address");
    let mut socket = SmartSocketServer::new(listener);
    socket.listen();
}
