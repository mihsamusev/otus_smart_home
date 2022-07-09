use crate::device::{Device, ProviderError};
use chrono::Utc;
use std::collections::HashMap;
use std::fmt::Write;
use std::rc::Rc;

pub struct Room {
    pub id: String,
    devices: HashMap<String, Rc<dyn Device>>,
}

impl Room {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.into(),
            devices: HashMap::new(),
        }
    }

    pub fn add_device(&mut self, device: Rc<dyn Device>) -> Option<Rc<dyn Device>> {
        self.devices.insert(device.get_id(), device)
    }

    pub fn remove_device(&mut self, id: &str) -> Option<Rc<dyn Device>> {
        self.devices.remove(id)
    }

    pub fn get_device(&self, id: &str) -> Result<Rc<dyn Device>, ProviderError> {
        if let Some(device) = self.devices.get(id) {
            Ok(device.clone())
        } else {
            Err(ProviderError::NoDeviceError(id.into()))
        }
    }

    pub fn get_device_ids(&self) -> Vec<String> {
        self.devices.keys().cloned().collect()
    }

    pub fn with_device(mut self, device: Rc<dyn Device>) -> Self {
        self.add_device(device);
        self
    }
}

pub struct SmartHome {
    pub id: String,
    rooms: HashMap<String, Room>,
}

impl SmartHome {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.into(),
            rooms: HashMap::new(),
        }
    }

    // default behaviour is to overwrite existing SmartHome rooms layout
    pub fn with_room(mut self, room: Room) -> Self {
        self.add_room(room);
        self
    }

    pub fn add_room(&mut self, room: Room) -> Option<Room> {
        let room_id = room.id.clone();
        self.rooms.insert(room_id, room)
    }

    pub fn remove_room(&mut self, id: &str) -> Option<Room> {
        self.rooms.remove(id)
    }

    pub fn get_room(&self, id: &str) -> Option<&Room> {
        self.rooms.get(id)
    }

    pub fn get_room_ids(&self) -> Vec<String> {
        self.rooms.keys().cloned().collect()
    }

    pub fn get_room_device_ids(&self, room_id: &str) -> Option<Vec<String>> {
        self.get_room(room_id).map(|room| room.get_device_ids())
    }

    // Generates structured report based on device statuses provided by
    // DeviceInfoProvider
    //
    pub fn get_status(&self) -> String {
        let datetime = Utc::now();
        let mut report = format!(
            "[SmartHome: {}] status on {}: \n",
            self.id,
            datetime.format("%Y-%m-%d %H:%M:%S")
        );

        for (room_id, room) in self.rooms.iter() {
            for (_, device) in room.devices.iter() {
                write!(report, "[ROOM '{}'] ", room_id).unwrap();
                match device.status() {
                    Ok(ok_status) => writeln!(report, "{}", ok_status).unwrap(),
                    Err(err_status) => writeln!(report, "{}", err_status).unwrap(),
                };
            }
        }
        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    struct TestDevice {
        id: String,
    }
    impl Device for TestDevice {
        fn get_id(&self) -> String {
            self.id.clone()
        }
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
        let device = Rc::new(TestDevice { id: "0".into() });
        let mut room = Room::new("room_1");
        assert!(room.add_device(device).is_none());
    }

    #[test]
    fn test_create_room_with_device() {
        let device = Rc::new(TestDevice { id: "0".into() });
        let room = Room::new("room_1").with_device(device);
        assert_eq!(room.devices.len(), 1);
    }

    #[test]
    fn test_create_room_with_device_add_another() {
        let device_1 = Rc::new(TestDevice { id: "0".into() });
        let device_2 = Rc::new(TestDevice { id: "1".into() });
        let mut room = Room::new("room_1").with_device(device_1);
        room.add_device(device_2);
        assert_eq!(room.devices.len(), 2);
    }

    #[test]
    fn test_create_room_with_device_add_duplicate() {
        let device_1 = Rc::new(TestDevice { id: "0".into() });
        let device_2 = Rc::new(TestDevice { id: "0".into() });
        let mut room = Room::new("room_1").with_device(device_1);
        assert!(room.add_device(device_2).is_some());
        assert_eq!(room.devices.len(), 1);
    }

    #[test]
    fn test_remove_device() {
        let device_1 = Rc::new(TestDevice { id: "0".into() });
        let device_2 = Rc::new(TestDevice { id: "1".into() });
        let mut room = Room::new("room_1")
            .with_device(device_1)
            .with_device(device_2);
        assert!(room.remove_device("1").is_some());
        assert_eq!(room.devices.len(), 1);
    }

    #[test]
    fn get_device() {
        let device_1 = Rc::new(TestDevice { id: "0".into() });
        let room = Room::new("room_1").with_device(device_1);
        let device = room.get_device("0");
        assert!(device.is_ok());

        let device = room.get_device("5");
        assert!(device.is_err());
    }
    #[test]
    fn test_get_room_device_ids() {
        let device_1 = Rc::new(TestDevice { id: "0".into() });
        let device_2 = Rc::new(TestDevice { id: "1".into() });
        let room = Room::new("room_1")
            .with_device(device_1)
            .with_device(device_2);
        let mut device_ids = room.get_device_ids();
        device_ids.sort();
        assert_eq!(device_ids, vec!["0", "1"]);
    }

    #[test]
    fn test_build_home_with_room() {
        let home = SmartHome::new("home")
            .with_room(Room::new("room_1"))
            .with_room(Room::new("room_2"));
        assert_eq!(home.rooms.len(), 2);
    }

    #[test]
    fn test_add_room_to_home() {
        let mut home = SmartHome::new("home");
        assert!(home.add_room(Room::new("room_1")).is_none());
        assert_eq!(home.rooms.len(), 1);
    }

    #[test]
    fn test_add_duplicate_room_to_home() {
        let mut home = SmartHome::new("home");
        home.add_room(Room::new("room_1"));
        assert!(home.add_room(Room::new("room_1")).is_some());
        assert_eq!(home.rooms.len(), 1);
    }
    #[test]
    fn test_remove_room_from_home() {
        let mut home = SmartHome::new("home")
            .with_room(Room::new("room_1"))
            .with_room(Room::new("room_2"));
        assert!(home.remove_room("room_2").is_some());
        assert_eq!(home.rooms.len(), 1);
    }

    #[test]
    fn get_home_room_ids() {
        let home = SmartHome::new("home")
            .with_room(Room::new("room_1"))
            .with_room(Room::new("room_2"));
        let mut room_ids = home.get_room_ids();
        room_ids.sort();
        assert_eq!(room_ids, vec!["room_1", "room_2"]);
    }

    #[test]
    fn test_get_home_room_device_ids() {
        let device_1 = Rc::new(TestDevice { id: "0".into() });
        let device_2 = Rc::new(TestDevice { id: "1".into() });
        let device_3 = Rc::new(TestDevice { id: "2".into() });
        let home = SmartHome::new("home")
            .with_room(
                Room::new("room_1")
                    .with_device(device_1)
                    .with_device(device_2),
            )
            .with_room(Room::new("room_2").with_device(device_3));

        let mut device_ids = home.get_room_device_ids("room_1").unwrap();
        device_ids.sort();
        assert_eq!(device_ids, vec!["0", "1"]);

        let device_ids = home.get_room_device_ids("room_2").unwrap();
        assert_eq!(device_ids, vec!["2"]);
    }
}
