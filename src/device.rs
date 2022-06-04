use std::fmt::Display;

pub trait DeviceInfoProvider {
    // todo: метод, возвращающий состояние устройства по имени комнаты и имени устройства
    fn status(&self, room_id: &str, device_id: &str) -> Option<String>;
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

impl Display for SmartSocket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state_str = if self.is_on { "on" } else { "off" };
        write!(
            f,
            "[DEVICE: '{}'] [STATUS] SmartSocket is {} and consumes {} W",
            self.id, state_str, self.power_used
        )
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

impl Display for SmartTermometer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[DEVICE: '{}'] [STATUS] SmartTermometer shows: {} °C",
            self.id, self.temperature
        )
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
