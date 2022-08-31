pub mod mock;
pub mod server;
pub mod tcp_socket_client;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProviderError {
    #[error("NoDeviceError: device with id '{0}' not provided!")]
    NoDeviceError(String),
    #[error("DeviceError: {0}")]
    DeviceError(#[from] DeviceError),
}

#[derive(Error, Debug)]
pub enum DeviceError {
    #[error("Thermometer error: {0}")]
    ThermometerError(String),
    #[error("SmartSocket error: {0}")]
    SocketError(String),
}

pub trait ReportableDevice {
    fn status(&self) -> Result<String, DeviceError>;
}

pub trait QueryableDevice {
    fn execute(&mut self, command: &str) -> Result<String, DeviceError>;
}
