use crate::device::mock::SmartSocket;
use crate::device::QueryableDevice;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

// socket server maps Tcp requests from multiple users to
// socket device commandsto simulate the device
//
pub struct SmartSocketServer {
    pub device: Arc<Mutex<SmartSocket>>,
    pub listener: TcpListener,
}

impl SmartSocketServer {
    pub fn new(listener: TcpListener) -> Self {
        let device = Arc::new(Mutex::new(SmartSocket::new()));
        Self { device, listener }
    }

    pub fn listen(&mut self) {
        println!(
            "[SmartSocket] listening on {}",
            &self.listener.local_addr().expect("Couldnt get local addr")
        );
        for stream in self.listener.incoming() {
            match stream {
                Err(e) => {
                    eprintln!("fail: {}", e)
                }
                Ok(stream) => {
                    let client_addr = stream.peer_addr().unwrap();
                    let socket_ref = self.device.clone();
                    thread::spawn(move || {
                        handle_smart_device(stream, socket_ref)
                            .unwrap_or_else(|_| eprintln!("{} disconnected", client_addr));
                    });
                }
            }
        }
    }
}

fn handle_smart_device(
    mut stream: TcpStream,
    device: Arc<Mutex<SmartSocket>>,
) -> Result<(), io::Error> {
    let client_addr = &stream.peer_addr()?;
    println!("[SmartDevice] {} connected", client_addr);

    loop {
        let mut buf: [u8; 10] = [0; 10];
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            println!("[SmartDevice] {} disconnected", client_addr);
            return Ok(());
        }

        let mut device = device.lock().unwrap();
        let command = std::str::from_utf8(&buf)
            .unwrap_or_default()
            .trim_matches(char::from(0))
            .trim();
        let mut response = match device.execute(command) {
            Ok(ok_resp) => ok_resp,
            Err(err_resp) => format!("{:?}", err_resp),
        };

        println!("[SmartDevice] {}: {}", client_addr, &response);

        // send response back to the strem
        response.push('\n');
        stream.write_all(response.as_bytes())?;
    }
}
