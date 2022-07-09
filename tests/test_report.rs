use smart_home::device::{SmartSocket, SmartTermometer};
use smart_home::home::{Room, SmartHome};
use std::rc::Rc;

#[test]
fn test_integration() {
    let socket1 = Rc::new(SmartSocket::new("sock_1"));
    let thermo1 = Rc::new(SmartTermometer::new("thermo_1"));

    let house = SmartHome::new("my_home")
        .with_room(Room::new("kitchen").with_device(socket1))
        .with_room(Room::new("garage").with_device(thermo1));
    let report = house.get_status();

    assert!(report.contains("[ROOM 'kitchen'] [DEVICE: 'sock_1'] [STATUS]"));
    assert!(report.contains("[ROOM 'garage'] [DEVICE: 'thermo_1'] [STATUS]"));
}
