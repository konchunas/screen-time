extern crate gtk;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;
extern crate relm_attributes;

use relm_attributes::widget;

use gtk::Orientation::*;
use gtk::{
    BoxExt, ButtonBoxExt, ButtonExt, FrameExt, GtkWindowExt, HeaderBarExt, Inhibit, OrientableExt,
    RadioButtonExt, ScrolledWindowExt, ToggleButtonExt, WidgetExt, LabelExt
};
use relm::{Component, ContainerWidget};
use relm::{Relm, Widget};

mod data;
mod desktop_info;
mod time_helper;
mod total_usage_widget;
mod usage_widget;

use crate::desktop_info::{load_as_apps, load_as_categories, AppInfo};
use crate::total_usage_widget::{Msg as TotalUsageMsg, TotalUsage};
use crate::usage_widget::UsageWidget;

static SHOW_CATEGORIES: &'static str = "Show categories";
static SHOW_APPS: &'static str = "Show apps";


#[derive(Msg)]
pub enum Msg {
    Add(AppInfo),
    ToggleCategoriesMode,
    ShowWeekStats,
    ShowDayStats,
    Quit,
}

pub struct Model {
    relm: Relm<Win>,
    usage_widgets: Vec<Component<UsageWidget>>,
    is_categories_mode: bool,
    days_count: i64,
}

#[widget]
impl Widget for Win {
    fn model(relm: &Relm<Self>, _: ()) -> Model {
        Model {
            relm: relm.clone(),
            usage_widgets: vec![],
            days_count: 1,
            is_categories_mode: false,
        }
    }

    fn subscriptions(&mut self, _: &Relm<Self>) {
        self.reload_stats();
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Add(app_info) => {
                let widget = self.most_used.add_widget::<UsageWidget>(app_info);
                self.model.usage_widgets.push(widget);
            }
            Msg::ShowDayStats => {
                if self.today_radio.get_active() {
                    self.model.days_count = 1;
                    self.reload_stats();
                }
            }
            Msg::ShowWeekStats => {
                if self.week_radio.get_active() {
                    self.model.days_count = 7;
                    self.reload_stats();
                }
            }
            Msg::ToggleCategoriesMode => {
                self.model.is_categories_mode = !self.model.is_categories_mode;
                self.reset_categories_switcher_text();
                self.reload_stats();
            }
            Msg::Quit => gtk::main_quit(),
        }
    }

    fn init_view(&mut self) {
        let header = gtk::HeaderBar::new();
        header.set_title("Screen Time");
        header.set_show_close_button(true);
        self.window.set_titlebar(&header);
        header.show();
        self.week_radio.join_group(Some(&self.today_radio));
        self.reset_categories_switcher_text();
    }

    view! {
        #[name="window"]
        gtk::Window {
            property_default_width: 500,
            gtk::Box {
                orientation: gtk::Orientation::Vertical,
                // spacing: 12,
                gtk::ButtonBox {
                    margin_top: 15,
                    orientation: gtk::Orientation::Horizontal,
                    layout: gtk::ButtonBoxStyle::Expand,
                    halign: gtk::Align::Center,
                    #[name="today_radio"]
                    gtk::RadioButton {
                        label: "Today",
                        mode: false,
                        toggled => Msg::ShowDayStats,
                    },
                    #[name="week_radio"]
                    gtk::RadioButton {
                        label: "7 days",
                        mode: false,
                        toggled => Msg::ShowWeekStats,
                    },
                },
                #[name="total_usage"]
                TotalUsage {
                    margin_top: 12,
                    margin_bottom: 12
                },

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
                    margin_left: 12,
                    margin_right: 12,
                    margin_bottom: 1,   
                },

                gtk::Frame {
                    // label: "Most used",
                    shadow_type: gtk::ShadowType::EtchedIn,
                    gtk::ScrolledWindow {
                        property_hscrollbar_policy: gtk::PolicyType::Never,
                        #[name="most_used"]
                        gtk::Box {
                            orientation: Vertical,
                        },

                        hexpand: true,
                        vexpand: true,
                        min_content_width: 350,
                        min_content_height: 350,

                    },
                    child: {
                        expand: true,
                        fill: true,
                    },
                    margin_left: 12,
                    margin_bottom: 12,
                    margin_right: 12,
                }
            },
            // Use a tuple when you want to both send a message and return a value to
            // the GTK+ callback.
            delete_event(_, _) => (Msg::Quit, Inhibit(false)),
        }
    }
}

impl Win {
    fn reset_categories_switcher_text(&mut self) {
        let text = match self.model.is_categories_mode {
            true => SHOW_APPS,
            false => SHOW_CATEGORIES
        };
        self.mode_switcher.set_markup(&format!("<a href=''>{}</a>", text));
    }

    fn reload_stats(&mut self) {
        while let Some(widget) = self.model.usage_widgets.pop() {
            self.most_used.remove_widget(widget);
        }
        self.load_stats(self.model.days_count, self.model.is_categories_mode);
    }

    fn load_stats(&mut self, days_count: i64, categories_mode: bool) {
        let frames = data::load_from_prev_days(days_count).unwrap();
        let (earliest, latest) = data::get_earliest_and_latest(&frames);

        self.total_usage
            .emit(TotalUsageMsg::SetSpan(earliest, latest));

        let entries = data::calculate_usage(frames);
        let total_usage = entries.iter().fold(0, |acc, entry| acc + entry.time);
        self.total_usage.emit(TotalUsageMsg::SetTotal(total_usage));
        let app_infos = match categories_mode {
            true => desktop_info::load_as_categories(entries, total_usage as f64),
            false => desktop_info::load_as_apps(entries, total_usage as f64)
        };
        for app_info in app_infos {
            self.model.relm.stream().emit(Msg::Add(app_info));
        }
    }
}

fn main() {
    Win::run(()).expect("Win::run failed");
}
