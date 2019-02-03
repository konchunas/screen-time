use gio::{AppInfoExt, DesktopAppInfo, DesktopAppInfoExt, Icon, ThemedIcon};
/// Searches and provides info from dektop files
use std::collections::HashMap;

use crate::data::UsageEntry;
use crate::time_helper::format_duration;
use crate::collected_app_info::COLLECTED_APPINFO;
use gtk::Cast;

// categories which provide some useful insight of activity
static SPECIFIC_CATEGORIES: &[&'static str] = &[
    "Development",
    "WebBrowser",
    "InstantMessaging",
    "Education",
    "Game",
    "Audio",
    "Video",
    "AudioVideo",
    "Finance",
    "Graphics",
    "Science",
    "TerminalEmulator",
];

// not so important categories to use as a second frontier
static SECONDARY_CATEGORIES: &[&'static str] = &[
    "Settings", "Network", "Office", "System", "Utility",
];

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
            None => (None, None),
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

    let sorted_category_usage_info = categories_usage
        .into_iter()
        .map(|entry| AppInfo {
            icon: get_category_icon(&entry.name),
            name: entry.name,
            fraction: entry.time as f64 / total_usage,
            duration: format_duration(entry.time),
        })
        .collect();

    sorted_category_usage_info
}

fn get_desktop_app_info(class_name: &str) -> Option<DesktopAppInfo> {
    if COLLECTED_APPINFO.contains_key(class_name) {
        let filename = COLLECTED_APPINFO.get(class_name).unwrap();
        let desktop_info = DesktopAppInfo::new_from_filename(filename);
        return Some(desktop_info);
    }

    let search_results = DesktopAppInfo::search(&class_name);
    if !search_results.is_empty() && !search_results[0].is_empty() {
        return Some(DesktopAppInfo::new(&search_results[0][0])); //take the first match
    }
    None
}

fn get_category(info: DesktopAppInfo) -> Option<String> {
    let categories = info.get_categories()?;

    let cat_list: Vec<&str> = categories.split(';').collect();
    for category in cat_list.iter() {
        if SPECIFIC_CATEGORIES
            .iter()
            .position(|cat| cat == category)
            .is_some()
        {
            return Some(category.to_string());
        }
    }

    for category in cat_list.iter() {
        if SECONDARY_CATEGORIES
            .iter()
            .position(|cat| cat == category)
            .is_some()
        {
            return Some(category.to_string());
        }
    }

    None
}

fn get_category_icon(name: &str) -> Option<Icon> {
    let icon_name = match name {
        "Development" => "applications-development",
        "WebBrowser" | "Network" => "applications-internet",
        "InstantMessaging" => "applications-fonts",
        "Education" => "applications-education",
        "Audio" | "Video" | "AudioVideo" => "applications-multimedia",
        "Finance" | "Office" => "applications-office",
        "Graphics" => "applications-graphics",
        "Science" => "applications-science",
        "Utility" => "applications-utilities",
        "TerminalEmulator" => "utilities-terminal",
        "System" | "Settings" => "applications-system",
        "Game" => "applicaions-games",
        _ => "applications-other",
    };

    let themed_icon = ThemedIcon::new(icon_name);
    let icon = themed_icon.upcast::<Icon>();
    Some(icon)
}
