use rand;
use serde::{Deserialize, Serialize};
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

// socket server maps Tcp requests from multiple users to
// socket device commandsto simulate the device
//
pub struct SmartSocketServer {
    pub device: Arc<Mutex<SmartSocketDevice>>,
    pub listener: TcpListener,
}

impl SmartSocketServer {
    pub fn new(listener: TcpListener) -> Self {
        let device = Arc::new(Mutex::new(SmartSocketDevice::new()));
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
    device: Arc<Mutex<SmartSocketDevice>>,
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
        let mut response = device.execute(command);

        println!("[SmartDevice] {}: {}", client_addr, &response);

        // send response back to the strem
        response.push('\n');
        stream.write(&response.as_bytes())?;
    }
}

#[derive(Serialize, Deserialize)]
pub struct SmartSocketDevice {
    enabled: bool,
    power: f32,
}

impl SmartSocketDevice {
    pub fn new() -> Self {
        Self {
            enabled: false,
            power: 0.0,
        }
    }

    pub fn update(&mut self) {
        if self.enabled {
            self.power = rand::random::<f32>();
        }
    }
    pub fn get_power_usage(&mut self) -> f32 {
        self.power
    }

    pub fn get_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_on(&mut self) {
        self.enabled = true
    }

    pub fn set_off(&mut self) {
        self.enabled = false;
        self.power = 0.0
    }

    pub fn get_status(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn execute(&mut self, query: &str) -> String {
        self.update();
        let status = match query {
            "SET1" => {
                self.set_on();
                self.get_status()
            }
            "SET0" => {
                self.set_off();
                self.get_status()
            }
            "GET" => self.get_status(),
            _ => String::new(),
        };
        status
    }
}
