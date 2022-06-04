use smart_home::{DeviceInfoProvider, SmartHome};
use std::fmt::{Display, Write};

// ***** Пример использования библиотеки умный дом:

// Пользовательские устройства:
pub struct SmartSocket {
    id: String,
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
    id: String,
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

// Пользовательские поставщики информации об устройствах.
// Могут как хранить устройства, так и заимствывать.
pub struct OwningDeviceInfoProvider {
    socket: SmartSocket,
}
pub struct BorrowingDeviceInfoProvider<'a, 'b> {
    socket: &'a SmartSocket,
    thermo: &'b SmartTermometer,
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn status(&self, room_id: &str, device_id: &str) -> Option<String> {
        let mut status_str = format!("[ROOM '{}'] ", room_id);

        if self.socket.id == device_id {
            write!(status_str, "{}", self.socket).unwrap();
        } else {
            write!(status_str, "Device with id '{}' not provided!", device_id).unwrap();
        }
        Some(status_str)
    }
}

impl<'a, 'b> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a, 'b> {
    fn status(&self, room_id: &str, device_id: &str) -> Option<String> {
        let mut status_str = format!("[ROOM '{}'] ", room_id);

        if self.socket.id == device_id {
            write!(status_str, "{}", self.socket).unwrap();
        } else if self.thermo.id == device_id {
            write!(status_str, "{}", self.thermo).unwrap();
        } else {
            write!(status_str, "Device with id '{}' not provided!", device_id).unwrap();
        }
        Some(status_str)
    }
}

fn main() {
    // Инициализация устройств
    let socket1 = SmartSocket::new("sock_1");
    let socket2 = SmartSocket::new("sock_2");
    let thermo = SmartTermometer::new("thermo_1");

    // Инициализация дома
    let house = SmartHome::new("my_home")
        .with_room("kitchen", &["sock_1", "sock_2"])
        .with_room("garage", &["thermo_1"]);

    println!("Initialized house '{}' with following layout:", house.id);
    for room in house.get_rooms() {
        println!("room: {}, device ids: {:?}", room, &house.devices(room));
    }
    println!();

    // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };
    let report1 = house.create_report(&info_provider_1);
    println!("Report #1:\n{report1}");

    // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    let info_provider_2 = BorrowingDeviceInfoProvider {
        socket: &socket2,
        thermo: &thermo,
    };
    let report2 = house.create_report(&info_provider_2);
    println!("Report #2:\n{report2}");
}
