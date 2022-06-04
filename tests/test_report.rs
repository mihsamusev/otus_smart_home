use smart_home::device::{DeviceInfoProvider, SmartSocket};
use smart_home::home::SmartHome;
use std::fmt::Write;

pub struct TestDevices {
    socket: SmartSocket,
}

impl DeviceInfoProvider for TestDevices {
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
#[test]
fn test_integration() {
    // Инициализация устройств
    let socket = SmartSocket::new("sock_1");
    // Инициализация дома
    let house = SmartHome::new("my_home")
        .with_room("kitchen", &["sock_1"])
        .with_room("garage", &["thermo_1"]);

    // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    let info_provider_1 = TestDevices { socket };
    let report = house.create_report(&info_provider_1);

    assert!(report.contains(
        "[ROOM 'kitchen'] [DEVICE: 'sock_1'] [STATUS] SmartSocket is off and consumes 0 W"
    ));
    assert!(report.contains("[ROOM 'garage'] Device with id 'thermo_1' not provided!"))
}
