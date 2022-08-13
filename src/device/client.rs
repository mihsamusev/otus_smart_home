use crate::device::{DeviceError, QueryableDevice, ReportableDevice};
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::net::{ToSocketAddrs, UdpSocket};
use std::str;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct TcpSmartSocket {
    address: String,
}

impl TcpSmartSocket {
    pub fn connect(address: &str) -> Self {
        Self {
            address: address.into(),
        }
    }

    pub fn get_query_doc() -> String {
        concat!(
            "[Smart socket]\n",
            "   'SET0' - turn off smart socket\n",
            "   'SET1' - turn on smart socket\n",
            "   'GET' - get smart socket state and power consumption\n"
        )
        .to_string()
    }
}

impl QueryableDevice for TcpSmartSocket {
    fn execute(&mut self, query: &str) -> Result<String, DeviceError> {
        let mut stream = TcpStream::connect(&self.address)
            .map_err(|e| DeviceError::SocketError(e.to_string()))?;

        // write a command to get status
        stream
            .write_all(query.as_bytes())
            .map_err(|e| DeviceError::SocketError(e.to_string()))?;

        // unpack the result
        let mut buf: Vec<u8> = Vec::new();
        let mut reader = BufReader::new(&stream);
        reader
            .read_until(b'\n', &mut buf)
            .map_err(|e| DeviceError::SocketError(e.to_string()))?;

        let response = str::from_utf8(&buf).unwrap_or_default();
        Ok(response.to_string())
    }
}

impl ReportableDevice for TcpSmartSocket {
    fn status(&self) -> Result<String, DeviceError> {
        let mut stream = TcpStream::connect(&self.address)
            .map_err(|e| DeviceError::SocketError(e.to_string()))?;

        // write a GET command to get status
        stream
            .write_all("GET".as_bytes())
            .map_err(|e| DeviceError::SocketError(e.to_string()))?;

        // unpack the result
        let mut buf: Vec<u8> = Vec::new();
        let mut reader = BufReader::new(&stream);
        reader
            .read_until(b'\n', &mut buf)
            .map_err(|e| DeviceError::SocketError(e.to_string()))?;

        let response = str::from_utf8(&buf).unwrap_or_default();
        Ok(response.to_string())
    }
}

pub struct UdpThermo {
    temperature: Arc<TemperatureData>,
    io_closed: Arc<AtomicBool>,
}

impl UdpThermo {
    pub fn listen(address: impl ToSocketAddrs) -> Result<Self, DeviceError> {
        let socket =
            UdpSocket::bind(address).map_err(|e| DeviceError::ThermoError(e.to_string()))?;
        socket
            .set_read_timeout(Some(Duration::from_secs(1)))
            .map_err(|e| DeviceError::ThermoError(e.to_string()))?;

        let io_closed = Arc::new(AtomicBool::new(false));
        let temperature = Arc::new(TemperatureData::default());

        let temperature_clone = temperature.clone();
        let io_closed_clone = io_closed.clone();

        thread::spawn(move || loop {
            if io_closed_clone.load(Ordering::SeqCst) {
                return;
            }

            let mut buf = [0; 4];
            if socket.recv_from(&mut buf).is_err() {
                temperature_clone.set(None);
            } else {
                let val = f32::from_be_bytes(buf);
                temperature_clone.set(Some(val));
            }
        });

        Ok(Self {
            temperature,
            io_closed,
        })
    }
}

impl Drop for UdpThermo {
    fn drop(&mut self) {
        // finish the listening thread
        self.io_closed.store(true, Ordering::SeqCst)
    }
}

impl QueryableDevice for UdpThermo {
    fn execute(&mut self, query: &str) -> Result<String, DeviceError> {
        match query {
            "GET" => self.status(),
            _ => Err(DeviceError::ThermoError(format!(
                "Unrecognized command {}",
                query
            ))),
        }
    }
}

impl ReportableDevice for UdpThermo {
    fn status(&self) -> Result<String, DeviceError> {
        match self.temperature.get() {
            Ok(t) => Ok(format!("{{temperature: {:.2}}}", t)),
            Err(e) => Err(e),
        }
    }
}

#[derive(Default)]
struct TemperatureData(Mutex<Option<f32>>);
impl TemperatureData {
    pub fn get(&self) -> Result<f32, DeviceError> {
        (*self.0.lock().unwrap()).ok_or_else(|| {
            DeviceError::ThermoError(
                "Failed to receive a temperature data from UDP socket".to_string(),
            )
        })
    }

    pub fn set(&self, value: Option<f32>) {
        *self.0.lock().unwrap() = value
    }
}
