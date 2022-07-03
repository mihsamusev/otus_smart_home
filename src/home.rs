use crate::device::Device;
use chrono::Utc;
use std::collections::HashMap;
use std::fmt::Write;

pub struct Room {
    pub id: String,
    devices: HashMap<String, Box<dyn Device>>,
}

impl Room {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.into(),
            devices: HashMap::new(),
        }
    }

    pub fn add_device(&mut self, id: &str, device: Box<dyn Device>) -> Option<Box<dyn Device>> {
        self.devices.insert(id.into(), device)
    }

    pub fn remove_device(&mut self, id: &str) -> Option<Box<dyn Device>> {
        self.devices.remove(id)
    }

    pub fn get_device(&self) {}

    pub fn get_devices(&self) {}

    pub fn with_device(mut self, id: &str, device: Box<dyn Device>) -> Self {
        self.add_device(id, device);
        self
    }
}
pub struct SmartHome {
    pub id: String,
    rooms: HashMap<String, Vec<Room>>,
}

impl SmartHome {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.into(),
            rooms: HashMap::new(),
        }
    }

    // default behaviour is to overwrite existing SmartHome rooms layout
    pub fn with_room(mut self, room_id: &str, device_ids: &[&str]) -> Self {
        let device_ids_vec = device_ids.iter().map(|x| x.to_string()).collect();
        self.rooms.insert(room_id.into(), device_ids_vec);
        self
    }
}
//     // Return a collection of room ids if any
//     //
//     pub fn get_rooms(&self) -> Option<Vec<&String>> {
//         match self.rooms.len() {
//             0 => None,
//             _ => Some(self.rooms.keys().collect()),
//         }
//     }

//     // Return a collection of device ids if any
//     //
//     pub fn devices(&self, room: &str) -> Option<&Vec<String>> {
//         self.rooms.get(room)
//     }

//     // Generates structured report based on device statuses provided by
//     // DeviceInfoProvider
//     //
//     pub fn create_report<T: DeviceInfoProvider>(&self, provider: &T) -> String {
//         let datetime = Utc::now();
//         let mut report = format!(
//             "[SmartHome: {}] status on {}: \n",
//             self.id,
//             datetime.format("%Y-%m-%d %H:%M:%S")
//         );

//         for (room_id, device_ids) in self.rooms.iter() {
//             for id in device_ids {
//                 write!(report, "[ROOM '{}'] ", room_id).unwrap();
//                 match provider.status(id) {
//                     Ok(ok_status) => writeln!(report, "{}", ok_status).unwrap(),
//                     Err(err_status) => writeln!(report, "{}", err_status).unwrap(),
//                 };
//             }
//         }
//         report
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    struct TestDevice {}
    impl Device for TestDevice {
        fn status(&self) -> Result<String, crate::device::DeviceError> {
            Ok("".into())
        }
    }
    #[test]
    fn test_create_room() {
        let room = Room::new("room_1");
        assert_eq!(room.id, "room_1");
        assert_eq!(room.devices.len(), 0);
    }

    #[test]
    fn test_add_device() {
        let device = Box::new(TestDevice {});
        let mut room = Room::new("room_1");
        assert!(room.add_device("device_1", device).is_none());
    }

    #[test]
    fn test_create_room_with_device() {
        let device = Box::new(TestDevice {});
        let room = Room::new("room_1").with_device("device_1", device);
        assert_eq!(room.devices.len(), 1);
    }

    #[test]
    fn test_create_room_with_device_add_another() {
        let device_1 = Box::new(TestDevice {});
        let device_2 = Box::new(TestDevice {});
        let mut room = Room::new("room_1").with_device("device_1", device_1);
        room.add_device("device_2", device_2);
        assert_eq!(room.devices.len(), 2);
    }

    #[test]
    fn test_create_room_with_device_add_duplicate() {
        let device_1 = Box::new(TestDevice {});
        let device_2 = Box::new(TestDevice {});
        let mut room = Room::new("room_1").with_device("device_1", device_1);
        assert!(room.add_device("device_1", device_2).is_some());
        assert_eq!(room.devices.len(), 1);
    }

    #[test]
    fn test_remove_device() {
        let device_1 = Box::new(TestDevice {});
        let device_2 = Box::new(TestDevice {});
        let mut room = Room::new("room_1")
            .with_device("device_1", device_1)
            .with_device("device_2", device_2);
        assert!(room.remove_device("device_1").is_some());
        assert_eq!(room.devices.len(), 1);
    }
}
//     #[test]
//     fn test_construct_home_with_a_room() {
//         let home = SmartHome::new("home_1").with_room("room_1", &["device_1", "device_2"]);

//         assert_eq!(
//             home.rooms.get("room_1"),
//             Some(&vec!["device_1".into(), "device_2".into()])
//         );
//     }

//     #[test]
//     fn test_get_home_rooms() {
//         let empty_home = SmartHome::new("home_0");
//         assert_eq!(empty_home.get_rooms(), None);

//         let full_home = SmartHome::new("home_1")
//             .with_room("room_1", &["device_1"])
//             .with_room("room_2", &["device_2"]);
//         let mut rooms = full_home.get_rooms().unwrap();
//         rooms.sort();
//         assert_eq!(rooms, vec!["room_1", "room_2"])
//     }

//     #[test]
//     fn test_get_room_devices() {
//         let home = SmartHome::new("home_1").with_room("room_1", &["device_1"]);

//         assert_eq!(home.devices("room_1"), Some(&vec!["device_1".into()]));
//         assert_eq!(home.devices("room_2"), None);
//     }
// }
