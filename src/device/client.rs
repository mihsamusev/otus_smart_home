use crate::device::{DeviceError, QueryableDevice, ReportableDevice};
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::str;

pub struct TcpSmartSocket {
    address: String,
}

impl TcpSmartSocket {
    pub fn connect(address: &str) -> Self {
        Self {
            address: address.into(),
        }
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
