use smart_home::device::client::{
    Device, DeviceInfoProvider, ProviderError, QueryableInfoProvider, TcpSmartSocket,
};

use smart_home::home::SmartHome;
use std::collections::HashMap;

pub enum NetworkedDeviceType {
    Socket(TcpSmartSocket),
}

pub struct NetworkedDevices {
    devices: HashMap<String, NetworkedDeviceType>,
}

impl DeviceInfoProvider for NetworkedDevices {
    fn status(&self, device_id: &str) -> Result<String, ProviderError> {
        if let Some(device) = self.devices.get(device_id) {
            match device {
                NetworkedDeviceType::Socket(d) => d.status().map_err(|e| e.into()),
            }
        } else {
            return Err(ProviderError::NoDeviceError(device_id.to_string()));
        }
    }
}

impl QueryableInfoProvider for NetworkedDevices {
    fn execute(&self, device_id: &str, command: &str) -> Result<String, ProviderError> {
        if let Some(device) = self.devices.get(device_id) {
            match device {
                NetworkedDeviceType::Socket(d) => d.execute(command).map_err(|e| e.into()),
            }
        } else {
            return Err(ProviderError::NoDeviceError(device_id.to_string()));
        }
    }
}

fn main() {
    // Initialize home
    let house = SmartHome::new("my_home").with_room("kitchen", &["sock_1"]);

    // initialize device provider
    let socket_address = "127.0.0.1:8888";
    let info_provider_1 = NetworkedDevices {
        devices: HashMap::from([(
            "sock_1".into(),
            NetworkedDeviceType::Socket(TcpSmartSocket::connect(&socket_address)),
        )]),
    };

    // Generate report on turned on device
    let report1 = house.create_report(&info_provider_1);
    println!("Report #1:\n{report1}");

    // send turn on query to smart socket
    let response = house.run_device_command(&info_provider_1, "kitchen/sock_1/SET1");
    println!(
        "Sending command 'kitchen/sock_1/SET1' to smart home -> {}",
        response
    );

    // Check report again
    let report2 = house.create_report(&info_provider_1);
    println!("Report #2:\n{report2}");
}
