use crate::device::mock::SmartSocket;
use crate::device::QueryableDevice;
use std::io;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

type SharedSmartSocket = Arc<Mutex<SmartSocket>>;

// socket server maps Tcp requests from multiple users to
// socket device commandsto simulate the device
//
pub struct SmartSocketServer {
    pub device: SharedSmartSocket,
    pub listener: TcpListener,
}

impl SmartSocketServer {
    pub fn new(listener: TcpListener) -> Self {
        let device = Arc::new(Mutex::new(SmartSocket::new()));
        Self { device, listener }
    }

    pub async fn listen(&mut self) {
        println!(
            "[SmartSocket] listening on {}",
            &self.listener.local_addr().expect("Couldnt get local addr")
        );
        loop {
            let connection = self.listener.accept().await;
            match connection {
                Err(e) => {
                    eprintln!("Connection failed: {}", e)
                }
                Ok((stream, client_addr)) => {
                    let socket_ref = self.device.clone();
                    tokio::spawn(async move {
                        handle_smart_device(stream, socket_ref)
                            .await
                            .unwrap_or_else(|_| eprintln!("{} disconnected", client_addr));
                    });
                }
            }
        }
    }
}

async fn handle_smart_device(
    mut stream: TcpStream,
    device: SharedSmartSocket,
) -> Result<(), io::Error> {
    let client_addr = &stream.peer_addr()?;
    println!("[SmartDevice] {} connected", client_addr);

    loop {
        let mut buf: [u8; 10] = [0; 10];
        let bytes_read = stream.read(&mut buf).await?;
        if bytes_read == 0 {
            println!("[SmartDevice] {} disconnected", client_addr);
            return Ok(());
        }

        let command = std::str::from_utf8(&buf)
            .unwrap_or_default()
            .trim_matches(char::from(0))
            .trim();

        let mut response = match device.lock().unwrap().execute(command) {
            Ok(ok_resp) => ok_resp,
            Err(err_resp) => format!("{:?}", err_resp),
        };
        println!("[SmartDevice] {}: {}", client_addr, &response);

        // send response back to the strem
        response.push('\n');
        stream.write_all(response.as_bytes()).await?;
    }
}
