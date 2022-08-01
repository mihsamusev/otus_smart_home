use crate::device::{DeviceError, ReportableDevice};
use rand;
use serde::{Deserialize, Serialize};

use super::QueryableDevice;

pub struct SmartTermometer {
    temperature: f32,
}

impl SmartTermometer {
    pub fn new() -> Self {
        Self { temperature: 0.0 }
    }
}

impl ReportableDevice for SmartTermometer {
    fn status(&self) -> Result<String, DeviceError> {
        Ok(format!("SmartTermometer shows: {} °C", self.temperature))
    }
}

impl Default for SmartTermometer {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Serialize, Deserialize)]
pub struct SmartSocket {
    enabled: bool,
    power: f32,
}

impl SmartSocket {
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

    pub fn get_power_usage(&self) -> f32 {
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

impl Default for SmartSocket {
    fn default() -> Self {
        Self::new()
    }
}

impl ReportableDevice for SmartSocket {
    fn status(&self) -> Result<String, DeviceError> {
        let state = if self.enabled { "on" } else { "off" };
        Ok(format!(
            "SmartSocket is {} and consumes 0 W {}",
            state,
            self.get_power_usage()
        ))
    }
}

impl QueryableDevice for SmartSocket {
    fn execute(&mut self, query: &str) -> Result<String, DeviceError> {
        self.update();
        match query {
            "SET1" => {
                self.set_on();
                Ok(self.get_status())
            }
            "SET0" => {
                self.set_off();
                Ok(self.get_status())
            }
            "GET" => Ok(self.get_status()),
            _ => Err(DeviceError::SocketError(format!(
                "Unrecognized command {}",
                query
            ))),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::assert_approx_eq;

    #[test]
    fn test_construct_socket() {
        let socket = SmartSocket::new();
        assert!(!socket.get_enabled());
        assert_approx_eq!(f32, socket.get_power_usage(), 0.0, epsilon = 0e-8)
    }

    #[test]
    fn test_construct_termometer() {
        let term = SmartTermometer::new();
        assert_approx_eq!(f32, term.temperature, 0.0, epsilon = 0e-8)
    }
}
