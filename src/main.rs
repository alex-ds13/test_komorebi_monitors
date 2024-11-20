use std::collections::{HashMap, VecDeque};

fn main() {
    let display_index_preferences = HashMap::from([
        (0, "DELA07A-7&ddb079d&0&UID265".into()),
        (1, "MSI3EA5-7&ddb079d&0&UID260".into()),
        (2, "DELA07B-7&ddb079d&0&UID256".into()),
    ]);
    let vd_new = VecDeque::new();
    let vd_old = VecDeque::new();
    let res = load_monitor_information(vd_new, vd_old, display_index_preferences);
    println!("Result New: {:#?}", res.0);
    println!("Result Old: {:#?}", res.1);
}

pub fn load_monitor_information(
    mut monitors_new: VecDeque<(usize, String, String)>,
    mut monitors_old: VecDeque<(usize, String, String)>,
    display_index_preferences: HashMap<usize, String>,
) -> (VecDeque<(usize, String, String)>, VecDeque<(usize, String, String)>) {
    'read: for display in win32_display_data::connected_displays_all().flatten() {
        let path = display.device_path.clone();

        let (device, device_id) = if path.is_empty() {
            (String::from("UNKNOWN"), String::from("UNKNOWN"))
        } else {
            let mut split: Vec<_> = path.split('#').collect();
            split.remove(0);
            split.remove(split.len() - 1);
            let device = split[0].to_string();
            let device_id = split.join("-");
            (device, device_id)
        };

        let name = display.device_name.trim_start_matches(r"\\.\").to_string();
        let name = name.split('\\').collect::<Vec<_>>()[0].to_string();

        let mut index_preference = None;
        for (index, id) in &display_index_preferences {
            if id.eq(&device_id) {
                index_preference = Option::from(index);
            }
        }

        if let Some(preference) = index_preference {
            while *preference >= monitors_new.len() {
                monitors_new.push_back((*preference, "PLACEHOLDER".into(), "".into()));
            }

            let current_name = monitors_new.get(*preference).map_or("", |(_idx, n, _id)| n);
            if current_name == "PLACEHOLDER" {
                let _ = monitors_new.remove(*preference);
                monitors_new.insert(*preference, (*preference, name.clone(), device_id.clone()));
            } else {
                monitors_new.insert(*preference, (*preference, name.clone(), device_id.clone()));
            }
        } else {
            monitors_new.push_back((usize::MAX, name.clone(), device_id.clone()));
        }

        if monitors_old.is_empty() {
            monitors_old.push_back((usize::MAX, name, device_id));
        } else if let Some(preference) = index_preference {
            while *preference > monitors_old.len() {
                monitors_old.push_back((*preference, "PLACEHOLDER".into(), "".into()));
            }

            monitors_old.insert(*preference, (*preference, name, device_id));
        } else {
            monitors_old.push_back((usize::MAX, name, device_id));
        }
    }

    monitors_new.retain(|m| m.1.ne("PLACEHOLDER"));
    monitors_old.retain(|m| m.1.ne("PLACEHOLDER"));

    (monitors_new, monitors_old)
}
