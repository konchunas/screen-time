use gio::Icon;
use crate::app_info::app_info_provider::AppInfoProvider;

pub struct WebAppInfo {
    name: String,
    icon: Icon,
    category: String,
}

impl AppInfoProvider for WebAppInfo {
    fn get_name(&self) -> Option<String> {
        Some(self.name.clone())
    }

    fn get_icon(&self) -> Option<Icon> {
        Some(self.icon.clone())
    }

    fn get_categories(&self) -> Option<String> {
        Some(self.category.clone())
    }
}

impl WebAppInfo {
    /*static */pub fn search(class_name: &str) -> Option<Self> {
        None
    }
}

// static web_apps: HashMap<&'static str, &WebAppInfo>  = ";";

