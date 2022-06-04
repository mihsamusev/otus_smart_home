use crate::device::DeviceInfoProvider;
use std::collections::HashMap;
use std::fmt::Write;

pub struct SmartHome {
    pub id: String,
    rooms: HashMap<String, Vec<String>>,
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

    pub fn get_rooms(&self) -> Vec<&String> {
        self.rooms.keys().collect()
    }

    pub fn devices(&self, room: &str) -> Option<&Vec<String>> {
        self.rooms.get(room)
    }

    pub fn create_report<T: DeviceInfoProvider>(&self, provider: &T) -> String {
        let mut report = format!("[SmartHome: {}] status: \n", self.id);

        for (room_id, device_ids) in self.rooms.iter() {
            for id in device_ids {
                if let Some(status) = provider.status(room_id, id) {
                    writeln!(report, "{}", status).unwrap();
                }
            }
        }
        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_empty_home() {
        let home = SmartHome::new("home_1");
        assert_eq!(home.id, "home_1");
        assert_eq!(home.rooms, HashMap::new());
    }

    #[test]
    fn test_construct_home_with_a_room() {
        let home = SmartHome::new("home_1").with_room("room_1", &["device_1", "device_2"]);

        assert_eq!(
            home.rooms.get("room_1"),
            Some(&vec!["device_1".into(), "device_2".into()])
        );
    }

    #[test]
    fn test_get_home_rooms() {
        let empty_home = SmartHome::new("home_0");
        assert_eq!(empty_home.get_rooms(), Vec::<&String>::new());

        let full_home = SmartHome::new("home_1")
            .with_room("room_1", &["device_1"])
            .with_room("room_2", &["device_2"]);
        let mut rooms = full_home.get_rooms();
        rooms.sort();
        assert_eq!(rooms, vec!["room_1", "room_2"])
    }

    #[test]
    fn test_get_room_devices() {
        let home = SmartHome::new("home_1").with_room("room_1", &["device_1"]);

        assert_eq!(home.devices("room_1"), Some(&vec!["device_1".into()]));
        assert_eq!(home.devices("room_2"), None);
    }
}
