use std::collections::HashMap;

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
            let device_statuses: Vec<String> = device_ids
                .iter()
                .filter_map(|device_id| provider.status(room_id, device_id))
                .collect();
            for status in device_statuses {
                report.push_str(&format!("{}\n", status));
            }
        }
        report
    }
}

pub trait DeviceInfoProvider {
    // todo: метод, возвращающий состояние устройства по имени комнаты и имени устройства
    fn status(&self, room_id: &str, device_id: &str) -> Option<String>;
}
