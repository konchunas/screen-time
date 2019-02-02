use gio::{AppInfoExt, DesktopAppInfo, DesktopAppInfoExt, Icon};
use std::collections::HashMap;

use crate::data::UsageEntry;
use crate::time_helper::format_duration;
use crate::app_info::AppInfoProvider;

// use gio::DesktopAppInfoExt;

//simple adapter for .desktop files info
impl AppInfoProvider for DesktopAppInfo {
    fn get_name(&self) -> Option<String> {
        gio::AppInfoExt::get_name(self)
    }

    fn get_icon(&self) -> Option<Icon> {
        gio::AppInfoExt::get_icon(self)
    }

    fn get_categories(&self) -> Option<String> {
        gio::DesktopAppInfoExt::get_categories(self)
    }
}