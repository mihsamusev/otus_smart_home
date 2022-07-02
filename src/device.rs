use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProviderError {
    #[error("NoDeviceError: device with id '{0}' not provided!")]
    NoDeviceError(String),
    #[error("DeviceError:")]
    DeviceError(#[from] DeviceError),
}

#[derive(Error, Debug)]
pub enum DeviceError {
    #[error("Thermometer error: {0}")]
    ThermometerError(String),
    #[error("SmartSocket error: {0}:")]
    SocketError(String),
}

pub trait Device {
    // can ask device all sorts of information through
    // the network IO
    fn status(&self) -> Result<String, DeviceError>;
}
pub trait DeviceInfoProvider {
    fn status(&self, device_id: &str) -> Result<String, ProviderError>;
}

pub struct SmartSocket {
    pub id: String,
    is_on: bool,
    power_used: f32,
}

impl SmartSocket {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.into(),
            is_on: false,
            power_used: 0.0,
        }
    }
}

impl Device for SmartSocket {
    fn status(&self) -> Result<String, DeviceError> {
        let state_str = if self.is_on { "on" } else { "off" };
        Ok(format!(
            "[DEVICE: '{}'] [STATUS] SmartSocket is {} and consumes {} W",
            self.id, state_str, self.power_used
        ))
    }
}

pub struct SmartTermometer {
    pub id: String,
    temperature: f32,
}

impl SmartTermometer {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.into(),
            temperature: 0.0,
        }
    }
}

impl Device for SmartTermometer {
    fn status(&self) -> Result<String, DeviceError> {
        Ok(format!(
            "[DEVICE: '{}'] [STATUS] SmartTermometer shows: {} °C",
            self.id, self.temperature
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::assert_approx_eq;

    #[test]
    fn test_construct_socket() {
        let socket = SmartSocket::new("id_1");
        assert_eq!(socket.id, "id_1");
        assert_eq!(socket.is_on, false);
        assert_approx_eq!(f32, socket.power_used, 0.0, epsilon = 0e-8)
    }

    #[test]
    fn test_construct_termometer() {
        let term = SmartTermometer::new("id_2");
        assert_eq!(term.id, "id_2");
        assert_approx_eq!(f32, term.temperature, 0.0, epsilon = 0e-8)
    }
}
