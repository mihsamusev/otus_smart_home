use smart_home::device::mock::SmartSocket;
use smart_home::device::{InfoDeviceProvider, ProviderError, ReportableDevice};
use smart_home::home::SmartHome;
use std::collections::HashMap;

pub enum DeviceType {
    Socket(SmartSocket),
}

pub struct TestDevices {
    devices: HashMap<String, DeviceType>,
}

impl InfoDeviceProvider for TestDevices {
    fn status(&self, device_id: &str) -> Result<String, ProviderError> {
        if let Some(device) = self.devices.get(device_id) {
            match device {
                DeviceType::Socket(d) => d.status().map_err(|e| e.into()),
            }
        } else {
            Err(ProviderError::NoDeviceError(device_id.to_string()))
        }
    }
}

#[test]
fn test_integration() {
    // Инициализация дома
    let house = SmartHome::new("my_home")
        .with_room("kitchen", &["sock_1"])
        .with_room("garage", &["thermo_1"]);

    // Строим отчёт
    let info_provider_1 = TestDevices {
        devices: HashMap::from([("sock_1".to_string(), DeviceType::Socket(SmartSocket::new()))]),
    };

    let report = house.create_report(&info_provider_1);
    dbg!(&report);
    assert!(report.contains(
        "[ROOM 'kitchen'] [DEVICE: 'sock_1'] [STATUS] SmartSocket is off and consumes 0 W"
    ));
    assert!(
        report.contains("[ROOM 'garage'] [DEVICE: 'thermo_1'] [STATUS] NoDeviceError: device with id 'thermo_1' not provided!")
    )
}
