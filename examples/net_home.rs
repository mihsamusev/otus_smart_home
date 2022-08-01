use smart_home::device::client::TcpSmartSocket;
use smart_home::device::{
    InfoDeviceProvider, ProviderError, QueryableDevice, QueryableDeviceProvider, ReportableDevice,
};
use smart_home::home::SmartHome;
use std::cell::RefCell;
use std::collections::HashMap;

pub enum DeviceType {
    TcpSocket(RefCell<TcpSmartSocket>),
}

pub struct MyDevices {
    devices: HashMap<String, DeviceType>,
}

impl InfoDeviceProvider for MyDevices {
    fn status(&self, device_id: &str) -> Result<String, ProviderError> {
        if let Some(device) = self.devices.get(device_id) {
            match device {
                DeviceType::TcpSocket(d) => d.borrow().status().map_err(|e| e.into()),
            }
        } else {
            Err(ProviderError::NoDeviceError(device_id.to_string()))
        }
    }
}

impl QueryableDeviceProvider for MyDevices {
    fn execute(&mut self, device_id: &str, command: &str) -> Result<String, ProviderError> {
        if let Some(device) = self.devices.get(device_id) {
            match device {
                DeviceType::TcpSocket(d) => d.borrow_mut().execute(command).map_err(|e| e.into()),
            }
        } else {
            Err(ProviderError::NoDeviceError(device_id.to_string()))
        }
    }
}

fn main() {
    // Initialize home
    let house = SmartHome::new("my_home").with_room("kitchen", &["sock_1"]);

    // initialize device provider
    let socket_address = "127.0.0.1:8888";
    let mut info_provider_1 = MyDevices {
        devices: HashMap::from([(
            "sock_1".to_string(),
            DeviceType::TcpSocket(RefCell::new(TcpSmartSocket::connect(socket_address))),
        )]),
    };

    // Generate report on turned on device
    let report1 = house.create_report(&info_provider_1);
    println!("Report #1:\n{report1}");

    // send turn on query to smart socket
    let response = house.run_device_query(&mut info_provider_1, "kitchen/sock_1/SET1");
    println!(
        "Sending command 'kitchen/sock_1/SET1' to smart home -> {}",
        response
    );

    // Check report again
    let report2 = house.create_report(&info_provider_1);
    println!("Report #2:\n{report2}");
}
