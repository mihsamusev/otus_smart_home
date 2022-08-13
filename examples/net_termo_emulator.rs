use rand::Rng;
use std::net::{SocketAddr, UdpSocket};
use std::thread;
use std::time::Duration;

fn main() {
    let address = "127.0.0.1:9001";
    // let sock = UdpSocket::bind(address).expect("fail");
    // let thermo_server = UdpThermoServer::new(sock);
    // thermo_server.run();

    let to_socket = address
        .parse::<SocketAddr>()
        .expect("Could not parse socket address");

    let bind_addr = "127.0.0.1:9000";
    let socket = UdpSocket::bind(bind_addr).expect("couldnt bind socket");
    println!("Will send date from {} to {}", bind_addr, to_socket);

    let mut rng = rand::thread_rng();
    loop {
        let temperature: f32 = rng.gen_range(-5.0..5.0);
        let send_result = socket.send_to(&temperature.to_be_bytes(), to_socket);
        if let Err(err) = send_result {
            println!("Unadble to send temperature: {err}")
        }
        thread::sleep(Duration::from_secs(1));
    }
}
