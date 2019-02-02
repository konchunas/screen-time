use relm::{Relm, Widget};
use relm_attributes::widget;
use relm::{Component, ContainerWidget};

use gtk::prelude::Inhibit;
use gtk::{BoxExt, WidgetExt, LabelExt, OrientableExt, FrameExt, ScrolledWindowExt};

use crate::data::UsageEntry;
use crate::usage_widget::UsageWidget;
use crate::desktop_info;

static SHOW_CATEGORIES: &'static str = "Show categories";
static SHOW_APPS: &'static str = "Show apps";

#[derive(Msg)]
pub enum Msg {
    Populate(Vec<UsageEntry>),
    Clear,
    ToggleCategoriesMode,
}

pub struct Model {
    usage_widgets: Vec<Component<UsageWidget>>,
    is_categories_mode: bool,
}

impl MostUsed {
    fn populate(&mut self, entries: Vec<UsageEntry>) {
        let total_usage = entries.iter().fold(0, |acc, entry| acc + entry.time);
        let app_infos = match self.model.is_categories_mode {
            true => desktop_info::load_as_categories(entries, total_usage as f64),
            false => desktop_info::load_as_apps(entries, total_usage as f64)
        };
        for app_info in app_infos {
            let widget = self.most_used.add_widget::<UsageWidget>(app_info);
            self.model.usage_widgets.push(widget);
        }
    }

    fn reset_categories_switcher_text(&mut self) {
        let text = match self.model.is_categories_mode {
            true => SHOW_APPS,
            false => SHOW_CATEGORIES
        };
        self.mode_switcher.set_markup(&format!("<a href=''>{}</a>", text));
    }
}

#[widget]
impl Widget for MostUsed {
    fn model(_: &Relm<Self>, _: ()) -> Model {
        Model {
            usage_widgets: vec![],
            is_categories_mode: false,
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Populate(entries) => {
                self.populate(entries);
            },
            Msg::Clear => {
                while let Some(widget) = self.model.usage_widgets.pop() {
                    self.most_used.remove_widget(widget);
                }
            },
            Msg::ToggleCategoriesMode => {
                self.model.is_categories_mode = !self.model.is_categories_mode;
                self.reset_categories_switcher_text();
            }
        }
    }

    fn init_view(&mut self) {
        self.reset_categories_switcher_text();
    }

    view! {
        gtk::Box {
            orientation: gtk::Orientation::Vertical,
            gtk::Box {
                orientation: gtk::Orientation::Horizontal,
                gtk::Label {
                    text: "Most used",
                },
                #[name="mode_switcher"]
                gtk::Label {
                    hexpand: true,
                    halign: gtk::Align::End,
                    track_visited_links: false,
                    activate_link(_,_) => (Msg::ToggleCategoriesMode, Inhibit(false)),
                },
                margin_left: 15,
                margin_right: 15,
                margin_bottom: 3,   
            },
            #[name="most_used_frame"]
            gtk::Frame {
                shadow_type: gtk::ShadowType::EtchedIn,
                gtk::ScrolledWindow {
                    property_hscrollbar_policy: gtk::PolicyType::Never,
                    #[name="most_used"]
                    gtk::Box {
                        orientation: gtk::Orientation::Vertical,
                    },

                    hexpand: true,
                    vexpand: true,
                    min_content_width: 400,
                    min_content_height: 400,

                },
                child: {
                    expand: true,
                    fill: true,
                },
                margin_left: 12,
                margin_bottom: 12,
                margin_right: 12,
            }
        }
    }
}
