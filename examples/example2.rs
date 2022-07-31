use smart_home::device::client::{
    Device, DeviceInfoProvider, ProviderError, QueryableInfoProvider, TcpSmartSocket,
};
use smart_home::device::server::SmartSocketServer;
use smart_home::home::SmartHome;
use std::collections::HashMap;
use std::net::TcpListener;
use std::thread;

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
    // get a listener with any available port
    // let listener = TcpListener::bind("127.0.0.1:0")
    //     .expect("Could not bind listener to any port!");
    // let socket_address = listener.local_addr().expect("Could read used port!").to_string();
    let socket_address = "127.0.0.1:8888";
    // let mut socket_server = SmartSocketServer::new(listener);
    // thread::spawn(move || socket_server.listen());

    // Инициализация дома
    let house = SmartHome::new("my_home").with_room("kitchen", &["sock_1"]);

    let info_provider_1 = NetworkedDevices {
        devices: HashMap::from([(
            "sock_1".into(),
            NetworkedDeviceType::Socket(TcpSmartSocket::connect(socket_address)),
        )]),
    };

    let report1 = house.create_report(&info_provider_1);
    println!("Report #1:\n{report1}");

    let response = house.run_device_command(&info_provider_1, "kitchen/sock_1/SET1");
    println!("Command #1:\n{response}");

    let report2 = house.create_report(&info_provider_1);
    println!("Report #2:\n{report2}");
}
