use std::{fmt::Display, io::Error};

struct SmartSocket {
    name: String,
    is_on: bool,
    power_used: f32,
}

impl SmartSocket {
    fn new(name: String) -> Self {
        Self {
            name,
            is_on: false,
            power_used: 0.0,
        }
    }

    fn _turn_on(&mut self) {
        todo!()
    }

    fn _turn_off(&mut self) {
        todo!()
    }

    fn _get_power_used(&self) {
        todo!()
    }
}

impl Display for SmartSocket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state_str = if self.is_on { "on" } else { "off" };
        write!(
            f,
            "[SmartSocket]: {} is {} and consumes {} W",
            self.name, state_str, self.power_used
        )
    }
}
struct SmartTermometer {
    name: String,
    temperature: f32,
}

impl SmartTermometer {
    fn new(name: String) -> Self {
        Self {
            name,
            temperature: 0.0,
        }
    }

    fn _get_temperature(&self) -> f32 {
        todo!()
    }
}

impl Display for SmartTermometer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[SmartTermometer]: {}  shows: {} °C",
            self.name, self.temperature
        )
    }
}

fn main() -> Result<(), Error> {
    let socket = SmartSocket::new("device_1".to_owned());
    println!("{}", socket);

    let termometer = SmartTermometer::new("device_2".to_owned());
    println!("{}", termometer);
    Ok(())
}
