use std::fmt::Display;

#[link(name = "socket", kind = "dylib")]
extern "C" {
    fn set_on(socket: Socket) -> Socket;
    fn set_off(socket: Socket) -> Socket;
    fn update_power(socket: Socket) -> Socket;
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Socket {
    enabled: bool,
    power: f32,
}

impl Default for Socket {
    fn default() -> Self {
        Self {
            enabled: false,
            power: 0.0,
        }
    }
}

impl Display for Socket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = if self.enabled { "on" } else { "off" };
        write!(
            f,
            "smart socket is '{state}' and consumes {:.2} W",
            self.power
        )
    }
}

impl Socket {
    pub fn set_on(&mut self) {
        let socket = unsafe { set_on(self.clone()) };
        *self = Socket { ..socket };
    }

    pub fn set_off(&mut self) {
        let socket = unsafe { set_off(self.clone()) };
        *self = Socket { ..socket };
    }

    pub fn update_power(&mut self) {
        let socket = unsafe { update_power(self.clone()) };
        *self = Socket { ..socket };
    }
}
