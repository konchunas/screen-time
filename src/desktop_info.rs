use gio::{AppInfoExt, DesktopAppInfo, DesktopAppInfoExt, Icon, ThemedIcon};
/// Searches and provides info from dektop files
use std::collections::HashMap;

use crate::data::UsageEntry;
use crate::time_helper::format_duration;
use gtk::Cast;

pub struct AppInfo {
    pub icon: Option<Icon>,
    pub name: String,
    pub duration: String,
    pub fraction: f64,
}

pub fn load_as_apps(entries: Vec<UsageEntry>, total_usage: f64) -> Vec<AppInfo> {
    let mut infos = vec![];
    for entry in entries {
        let (name, icon) = match get_desktop_app_info(&entry.name) {
            Some(info) => (info.get_name(), info.get_icon()),
            None => (None, None)
        };
        infos.push(AppInfo {
            name: name.unwrap_or(entry.name),
            duration: format_duration(entry.time),
            fraction: entry.time as f64 / total_usage,
            icon,
        });
    }
    return infos;
}

pub fn load_as_categories(entries: Vec<UsageEntry>, total_usage: f64) -> Vec<AppInfo> {
    let usage_by_categories = entries.into_iter().map(|entry| {
        let category = get_desktop_app_info(&entry.name).and_then(|info| get_category(info));
        UsageEntry {
            name: category.unwrap_or("Other".to_string()),
            time: entry.time,
        }
    });

    let usage_map = usage_by_categories.fold(HashMap::new(), |mut acc, usage_entry| {
        let counter = acc.entry(usage_entry.name).or_insert(0);
        *counter += usage_entry.time;
        acc
    });

    let mut categories_usage: Vec<UsageEntry> = usage_map
        .into_iter()
        .map(|(name, time)| UsageEntry { name, time })
        .collect();
    
    categories_usage.sort_unstable_by(|a, b| b.time.cmp(&a.time));
    
    let sorted_category_usage_info = categories_usage.into_iter().map(|entry| {
        AppInfo {
            icon: get_category_icon(&entry.name),
            name: entry.name,
            fraction: entry.time as f64 / total_usage,
            duration: format_duration(entry.time),
        }
    }).collect();

    sorted_category_usage_info
}

fn get_desktop_app_info(class_name: &str) -> Option<DesktopAppInfo> {
    let search_results = DesktopAppInfo::search(&class_name);
    if !search_results.is_empty() && !search_results[0].is_empty() {
        return Some(DesktopAppInfo::new(&search_results[0][0])); //take the first match
    }
    None
}

fn get_category(info: DesktopAppInfo) -> Option<String> {
    let category = info.get_categories();
    category
}

fn get_category_icon(name: &str) -> Option<Icon> {
    let icon_name = match name {
        _ => "applications-other",
    };

    let themed_icon = ThemedIcon::new(icon_name);
    let icon = themed_icon.upcast::<Icon>();
    Some(icon)
}