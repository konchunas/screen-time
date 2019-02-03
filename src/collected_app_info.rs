use std::collections::HashMap;
use crate::data;

lazy_static! {
    pub static ref COLLECTED_APPINFO: HashMap<String, String> = {
        match data::load_collected_app_info() {
            Ok(map) => map,
            Err(_) => HashMap::new()
        }
    };
}