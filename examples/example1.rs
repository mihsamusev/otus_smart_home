use smart_home::device::{Device, DeviceInfoProvider, ProviderError, SmartSocket, SmartTermometer};
use smart_home::home::SmartHome;
// ***** Пример использования библиотеки умный дом:

// Пользовательские устройства:

// Пользовательские поставщики информации об устройствах.
// Могут как хранить устройства, так и заимствывать.
pub struct MyDevices {
    socket: SmartSocket,
    thermo: SmartTermometer,
}

impl DeviceInfoProvider for MyDevices {
    fn status(&self, device_id: &str) -> Result<String, ProviderError> {
        if self.socket.id == device_id {
            self.socket.status().map_err(|e| e.into())
        } else if self.thermo.id == device_id {
            self.thermo.status().map_err(|e| e.into())
        } else {
            return Err(ProviderError::NoDeviceError(device_id.into()));
        }
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
    for room in house.get_rooms().unwrap() {
        println!("room: {}, device ids: {:?}", room, &house.devices(room));
    }
    println!();

    // Строим отчёт
    let info_provider_1 = MyDevices {
        socket: socket1,
        thermo: thermo1,
    };
    let report1 = house.create_report(&info_provider_1);
    println!("Report #1:\n{report1}");
}
