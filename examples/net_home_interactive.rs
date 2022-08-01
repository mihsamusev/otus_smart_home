use smart_home::device::client::TcpSmartSocket;
use smart_home::device::{
    InfoDeviceProvider, ProviderError, QueryableDevice, QueryableDeviceProvider, ReportableDevice,
};
use smart_home::home::SmartHome;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io;

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

fn get_help() -> String {
    concat!(
        "General commands:\n",
        "   'help' - to get this help text\n",
        "   'quit' - quit the smart home app\n",
        "   'report' - get full report over home devices\n",
        "Build a device query path using following template:\n",
        "   '<room_id>/<device_id>/<device_query>' for example 'kitchen/sock_1/GET'\n",
        "Available device queries:\n",
        "[Smart socket]\n",
        "   'SET0' - turn off smart socket\n",
        "   'SET1' - turn on smart socket\n",
        "   'GET' - get smart socket state and power consumption\n"
    )
    .to_string()
}
fn main() {
    // Initialize home
    let home = SmartHome::new("my_home").with_room("kitchen", &["sock_1"]);

    // initialize device provider
    let socket_address = "127.0.0.1:8888";
    let mut info_provider_1 = MyDevices {
        devices: HashMap::from([(
            "sock_1".to_string(),
            DeviceType::TcpSocket(RefCell::new(TcpSmartSocket::connect(socket_address))),
        )]),
    };

    println!("Connected to smart home: '{}' ", &home.id);
    println!("{}", get_help());

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line, shutting down...");

        let input = input.trim();
        match input {
            "quit" => {
                println!("Bye!");
                break;
            }
            "help" => {
                println!("{}", get_help());
            }
            "report" => {
                let report = home.create_report(&info_provider_1);
                println!("Report: {}", report);
            }
            _ => {
                let response = home.run_device_query(&mut info_provider_1, input);
                println!("{}", response)
            }
        }
    }
}
