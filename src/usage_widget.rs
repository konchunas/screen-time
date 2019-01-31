use relm_attributes::widget;
use relm::{Relm, Widget};

use gtk::{BoxExt, OrientableExt, LabelExt, ProgressBarExt, ImageExt, WidgetExt};
use gtk::Orientation::*;

use gio::{DesktopAppInfo, Icon};

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
        // let (name, icon) = get_display_name_and_icon(name);
        let (name, icon) = (name, None);
        let duration = format_duration(duration);
        Model {
            name,
            fraction,
            icon,
            duration,
        }
    }

    fn update(&mut self, event: Msg) {
        // match event {
        //     // A call to self.label1.set_text() is automatically inserted by the
        //     // attribute every time the model.counter attribute is updated.
        //     Msg::Decrement => self.model.counter -= 1,
        //     Msg::Increment => self.model.counter += 1,
        //     Msg::Quit => gtk::main_quit(),
        // }
    }

    fn init_view(&mut self) {
        use relm::ToGlib; //TODO after update
        self.icon.set_from_icon_name("unknown", gtk::IconSize::Dialog.to_glib());
    }

    view! {
        gtk::Box {
            orientation: Vertical,
            gtk::Box {
                orientation: Horizontal,
                spacing: 12,
                #[name="icon"]
                gtk::Image {},
                gtk::Box {
                    orientation: Vertical,
                    gtk::Label {
                        text: &self.model.name,
                    },
                    gtk::ProgressBar {
                        fraction: self.model.fraction,
                    },
                    child: {
                        expand: true,
                        padding: 12,
                    },
                },
                gtk::Label {
                    text: &self.model.duration,
                }
            },
            gtk::Separator {},
        }
    }
}

fn get_desktop_app_info_compat(class_name: &str) -> Option<DesktopAppInfo> {
    let search_results = DesktopAppInfo::search(&class_name);
    if !search_results.is_empty() && !search_results[0].is_empty() {
        println!("But found closest match {}", search_results[0][0]);
        return Ok(DesktopAppInfo::new(&search_results[0][0])) //take the first match
    }
    // DesktopAppInfo::new(&format!("{}.desktop", &class_name)).or_else(|| {
    //     println!("No match for {}.desktop", &class_name);
    //     let search_results = DesktopAppInfo::search(&class_name);
    //     if !search_results.is_empty() && !search_results[0].is_empty() {
    //         println!("But found closest match {}", search_results[0][0]);
    //         return DesktopAppInfo::new(&search_results[0][0]); //take the first match
    //     }
    //     None
    // })
    None

// fn get_display_name_and_icon(name: &str) -> (String, Option<gio::Icon>) {
//     let desktop_info = get_desktop_app_info(name);

//     let (name, icon) = match desktop_info {
//         Some(info) => (info.get_name(), info.get_icon()),
//         None => (None, None),
//     };

//     let name = name.map_or(name.to_string(), |name| String::from(name.as_str()));

//     (name, icon)
// }

// fn get_desktop_app_info(class_name: &str) -> Option<DesktopAppInfo> {
//     // DesktopAppInfo::new(&format!("{}.desktop", &class_name)).or_else(|| {
//     //     println!("No match for {}.desktop", &class_name);
//     //     let search_results = DesktopAppInfo::search(&class_name);
//     //     if !search_results.is_empty() && !search_results[0].is_empty() {
//     //         println!("But found closest match {}", search_results[0][0]);
//     //         return DesktopAppInfo::new(&search_results[0][0]); //take the first match
//     //     }
//     //     None
//     // })
//     None
// }
