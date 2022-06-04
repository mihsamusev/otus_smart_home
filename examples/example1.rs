use smart_home::device::{DeviceInfoProvider, SmartSocket, SmartTermometer};
use smart_home::home::SmartHome;
use std::fmt::Write;

// ***** Пример использования библиотеки умный дом:

// Пользовательские устройства:

// Пользовательские поставщики информации об устройствах.
// Могут как хранить устройства, так и заимствывать.
pub struct MyDevices {
    socket: SmartSocket,
    thermo: SmartTermometer,
}

impl DeviceInfoProvider for MyDevices {
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
    let thermo1 = SmartTermometer::new("thermo_1");

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
    let info_provider_1 = MyDevices {
        socket: socket1,
        thermo: thermo1,
    };
    let report1 = house.create_report(&info_provider_1);
    println!("Report #1:\n{report1}");
}
