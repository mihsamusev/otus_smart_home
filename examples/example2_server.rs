use std::net::{TcpStream, TcpListener};
use std::io::{self, Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use rand;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SmartSocketClient {
    enabled: bool,
    power: f32
}

impl SmartSocketClient {
    pub fn new() -> Self {
        Self {enabled: false, power: 0.0}
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
}

fn handle_connection(mut stream: TcpStream) -> Result<(), io::Error>{
    
    let mut smart_socket = SmartSocketClient::new();
    // read from the stream until the length of bytes is 0
    println!("{} connected", &stream.peer_addr()?);
    loop {
        let mut buf: [u8; 10] = [0; 10];
        let bytes_read = stream.read(&mut buf)?; 
        if bytes_read == 0 {
            return Ok(());
        }
        
        smart_socket.update();
        let command = std::str::from_utf8(&buf).unwrap_or_default().trim_matches(char::from(0)).trim();        
        let mut status = match command {
            "SET1" => {smart_socket.set_on(); smart_socket.get_status()},
            "SET0" => {smart_socket.set_off(); smart_socket.get_status()},
            "GET" => {smart_socket.get_status()}
            _ => String::new()
        };

        status.push('\n');
        stream.write(&status.as_bytes())?;
        // write buffer at read length back
    }
}

fn handle_device(mut stream: TcpStream, smart_socket: Arc<Mutex<SmartSocketClient>>) -> Result<(), io::Error>{
    
    //let mut smart_socket = SmartSocketClient::new();
    // read from the stream until the length of bytes is 0
    let client_addr = &stream.peer_addr()?;
    println!("{} connected", client_addr);
    loop {
        let mut buf: [u8; 10] = [0; 10];
        let bytes_read = stream.read(&mut buf)?; 
        if bytes_read == 0 {
            println!("{} disconnected", client_addr);
            return Ok(());
        }
        
        let mut device = smart_socket.lock().unwrap();

        device.update();
        let command = std::str::from_utf8(&buf).unwrap_or_default().trim_matches(char::from(0)).trim();        
        let mut status = match command {
            "SET1" => {device.set_on(); device.get_status()},
            "SET0" => {device.set_off(); device.get_status()},
            "GET" => {device.get_status()}
            _ => String::new()
        };

        status.push('\n');
        stream.write(&status.as_bytes())?;
        // write buffer at read length back
    }
}

fn main() {
    // create a tcp listener and bind it to a port
    let smart_socket = Arc::new(Mutex::new(SmartSocketClient::new()));
    let listener = TcpListener::bind("127.0.0.1:8888").expect("Could not bind");
    println!("Listening on {}", listener.local_addr().unwrap());

    for stream in listener.incoming() {
        match stream {
        Err(e) => {eprintln!("fail: {}", e)},
        Ok(stream) => {
            let client_addr = stream.peer_addr().unwrap();
            let socket_ref = smart_socket.clone();
            thread::spawn(move || {
                handle_device(stream, socket_ref).unwrap_or_else(|_| eprintln!("{} disconnected", client_addr));
            });
        }
    }
}
    // for each incomming stream:
    // create a thread that handles the connection
}
