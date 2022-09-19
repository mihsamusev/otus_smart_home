use crate::device::DeviceError;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::str;

pub fn query(address: impl ToSocketAddrs, query: &str) -> Result<String, DeviceError> {
    let mut stream =
        TcpStream::connect(&address).map_err(|e| DeviceError::SocketError(e.to_string()))?;

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

    let response = str::from_utf8(&buf).unwrap_or_default().trim();
    Ok(response.to_string())
}
