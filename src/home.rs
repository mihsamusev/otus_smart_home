use crate::device::client::{DeviceInfoProvider, QueryableInfoProvider};
use chrono::Utc;
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

    // Return a collection of room ids if any
    //
    pub fn get_rooms(&self) -> Option<Vec<&String>> {
        match self.rooms.len() {
            0 => None,
            _ => Some(self.rooms.keys().collect()),
        }
    }

    // Return a collection of device ids if any
    //
    pub fn devices(&self, room: &str) -> Option<&Vec<String>> {
        self.rooms.get(room)
    }

    // Generates structured report based on device statuses provided by
    // DeviceInfoProvider
    //
    pub fn create_report<T: DeviceInfoProvider>(&self, provider: &T) -> String {
        let datetime = Utc::now();
        let mut report = format!(
            "[SmartHome: {}] status on {}: \n",
            self.id,
            datetime.format("%Y-%m-%d %H:%M:%S")
        );

        for (room_id, device_ids) in self.rooms.iter() {
            for device_id in device_ids {
                write!(
                    report,
                    "[ROOM '{}'] [DEVICE: '{}'] [STATUS] ",
                    room_id, device_id
                )
                .unwrap();
                match provider.status(device_id) {
                    Ok(ok_status) => writeln!(report, "{}", ok_status).unwrap(),
                    Err(err_status) => writeln!(report, "{}", err_status).unwrap(),
                };
            }
        }
        report
    }

    pub fn run_device_command<T>(&self, provider: &T, command_query: &str) -> String
    where
        T: DeviceInfoProvider + QueryableInfoProvider + 'static,
    {
        let query_parts: Vec<&str> = command_query.split("/").collect();
        let room = query_parts[0];
        let device = query_parts[1];
        let command = query_parts[2];

        let mut response = String::new();
        if let Some(device_ids) = self.rooms.get(room) {
            if device_ids.contains(&device.to_string()) {
                match provider.execute(device, command) {
                    Ok(ok_status) => writeln!(response, "{}", ok_status).unwrap(),
                    Err(err_status) => writeln!(response, "{}", err_status).unwrap(),
                }
            }
        }
        response
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
        assert_eq!(empty_home.get_rooms(), None);

        let full_home = SmartHome::new("home_1")
            .with_room("room_1", &["device_1"])
            .with_room("room_2", &["device_2"]);
        let mut rooms = full_home.get_rooms().unwrap();
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
