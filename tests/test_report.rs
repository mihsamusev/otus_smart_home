use smart_home::device::{Device, DeviceInfoProvider, ProviderError, SmartSocket};
use smart_home::home::SmartHome;

pub struct TestDevices {
    socket: SmartSocket,
}

impl DeviceInfoProvider for TestDevices {
    fn status(&self, device_id: &str) -> Result<String, ProviderError> {
        if self.socket.id == device_id {
            self.socket.status().map_err(|e| e.into())
        } else {
            Err(ProviderError::NoDeviceError(device_id.into()))
        }
    }
}
#[test]
fn test_integration() {
    // Инициализация устройств
    let socket = SmartSocket::new("sock_1");

    // Инициализация дома
    let house = SmartHome::new("my_home")
        .with_room("kitchen", &["sock_1"])
        .with_room("garage", &["thermo_1"]);

    // Строим отчёт
    let info_provider_1 = TestDevices { socket };
    let report = house.create_report(&info_provider_1);
    dbg!(&report);
    assert!(report.contains(
        "[ROOM 'kitchen'] [DEVICE: 'sock_1'] [STATUS] SmartSocket is off and consumes 0 W"
    ));
    assert!(
        report.contains("[ROOM 'garage'] NoDeviceError: device with id 'thermo_1' not provided!")
    )
}
