use relm_attributes::widget;
use relm::{Relm, Widget};

use gtk::{BoxExt, OrientableExt, LabelExt, ProgressBarExt, ImageExt, WidgetExt};
use gtk::Orientation::*;

use gio::{DesktopAppInfo, AppInfoExt, Icon};

use crate::time_helper::format_duration;

#[derive(Msg)]
pub enum Msg {
    Update,
}

pub struct Model {
    icon: Option<Icon>,
    name: String,
    duration: String,
    fraction: f64
}

#[widget]
impl Widget for UsageWidget {
    fn model(_: &Relm<Self>, (name, duration, fraction): (String, i64, f64)) -> Model {
        let (name, icon) = get_display_name_and_icon(&name);
        let duration = format_duration(duration);
        Model {
            name,
            fraction,
            icon,
            duration,
        }
    }

    fn update(&mut self, _event: Msg) {
    }

    fn init_view(&mut self) {
        use relm::ToGlib; //TODO after update
        let icon_size = gtk::IconSize::Dialog.to_glib();
        match &self.model.icon {
            Some(icon) => self.icon.set_from_gicon(icon, icon_size),
            None => self.icon.set_from_icon_name("unknown", icon_size)
        }
    }

    view! {
        gtk::Box {
            orientation: Vertical,
            gtk::Box {
                orientation: Horizontal,
                spacing: 12,
                #[name="icon"]
                gtk::Image {
                    margin_left: 12,
                    margin_right: 6,
                    margin_top: 6,
                    margin_bottom: 6,
                },
                gtk::Box {
                    orientation: Vertical,
                    gtk::Label {
                        text: &self.model.name,
                        valign: gtk::Align::Fill,
                    },
                    gtk::ProgressBar {
                        fraction: self.model.fraction,
                        valign: gtk::Align::Center,
                        child: {
                            expand: true
                        }
                    },
                    // child: {
                    //     expand: true,
                    //     padding: 12,
                    // },
                    hexpand: true
                },
                gtk::Label {
                    text: &self.model.duration,
                    width_chars: 7,
                }
            },
            gtk::Separator {},
        }
    }
}

fn get_desktop_app_info(class_name: &str) -> Option<DesktopAppInfo> {
    let search_results = DesktopAppInfo::search(&class_name);
    if !search_results.is_empty() && !search_results[0].is_empty() {
        return Some(DesktopAppInfo::new(&search_results[0][0])) //take the first match
    }
    None
}

fn get_display_name_and_icon(class_name: &str) -> (String, Option<gio::Icon>) {
    let desktop_info = get_desktop_app_info(class_name);

    let (name, icon) = match desktop_info {
        Some(info) => (info.get_name(), info.get_icon()),
        None => (None, None),
    };

    let name = name.map_or(class_name.to_string(), |name| String::from(name.as_str()));

    (name, icon)
}
