use relm::{Relm, Widget};
use relm_attributes::widget;

use gtk::Orientation::*;
use gtk::{BoxExt, ImageExt, LabelExt, OrientableExt, ProgressBarExt, WidgetExt};

use crate::desktop_info::AppInfo as Model;
// use crate::time_helper::format_duration;

// #[derive(Msg)]
// pub enum Msg {
//     Update,
// }

#[widget]
impl Widget for UsageWidget {
    fn model(_: &Relm<Self>, model: Model) -> Model {
        model
    }

    fn update(&mut self, _event: ()) {}

    fn init_view(&mut self) {
        use relm::ToGlib; //TODO after update
        let icon_size = gtk::IconSize::Dialog.to_glib();
        match &self.model.icon {
            Some(icon) => self.icon.set_from_gicon(icon, icon_size),
            None => self.icon.set_from_icon_name("unknown", icon_size),
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
                    margin_right: 9,
                    margin_top: 9,
                    margin_bottom: 9,
                },
                gtk::Box {
                    orientation: Vertical,
                    gtk::Label {
                        text: &self.model.name,
                        child: {
                            expand: true,
                        },
                    },
                    gtk::ProgressBar {
                        fraction: self.model.fraction,
                        valign: gtk::Align::Center,
                        child: {
                            expand: true,
                        },
                    },
                    hexpand: true
                },
                gtk::Label {
                    text: &self.model.duration,
                    width_chars: 7,
                    margin_right: 12,
                }
            },
            gtk::Separator {},
        }
    }
}
