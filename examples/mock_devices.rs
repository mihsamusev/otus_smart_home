use smart_home::device::mock::{SmartSocket, SmartTermometer};
use smart_home::device::{InfoDeviceProvider, ProviderError, ReportableDevice};
use smart_home::home::SmartHome;
use std::collections::HashMap;

pub enum DeviceType {
    Socket(SmartSocket),
    Termo(SmartTermometer),
}

pub struct MyDevices {
    devices: HashMap<String, DeviceType>,
}

impl InfoDeviceProvider for MyDevices {
    fn status(&self, device_id: &str) -> Result<String, ProviderError> {
        if let Some(device) = self.devices.get(device_id) {
            match device {
                DeviceType::Socket(d) => d.status().map_err(|e| e.into()),
                DeviceType::Termo(d) => d.status().map_err(|e| e.into()),
            }
        } else {
            Err(ProviderError::NoDeviceError(device_id.to_string()))
        }
    }
}

fn main() {
    // Initalize devices
    let info_provider_1 = MyDevices {
        devices: HashMap::from([
            ("sock_1".to_string(), DeviceType::Socket(SmartSocket::new())),
            (
                "thermo_1".to_string(),
                DeviceType::Termo(SmartTermometer::new()),
            ),
        ]),
    };

    // Initialize home
    let house = SmartHome::new("my_home")
        .with_room("kitchen", &["sock_1", "sock_2"])
        .with_room("garage", &["thermo_1"]);

    println!("Initialized house '{}' with following layout:", house.id);
    for room in house.get_rooms().unwrap() {
        println!("room: {}, device ids: {:?}", room, &house.devices(room));
    }
    println!();

    // build report
    let report1 = house.create_report(&info_provider_1);
    println!("Report #1:\n{report1}");
}
