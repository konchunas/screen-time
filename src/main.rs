extern crate gtk;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;
extern crate relm_attributes;

use relm_attributes::widget;

use gtk::{
    ButtonBoxExt, ButtonExt, GtkWindowExt, HeaderBarExt, Inhibit,
    OrientableExt, RadioButtonExt, ToggleButtonExt, WidgetExt,
};
use relm::{Relm, Widget};

mod data;
mod most_used_widget;
mod time_helper;
mod total_usage_widget;
mod usage_widget;
mod app_info;

use crate::most_used_widget::{Msg as MostUsedMsg, MostUsed};
use crate::total_usage_widget::{Msg as TotalUsageMsg, TotalUsage};

use crate::most_used_widget::Msg::ToggleCategoriesMode as CategoriesModeToggled;

#[derive(Msg)]
pub enum Msg {
    ShowWeekStats,
    ShowDayStats,
    ToggleCategoriesMode,
    Quit,
}

pub struct Model {
    relm: Relm<Win>,
    days_count: i64,
}

#[widget]
impl Widget for Win {
    fn model(relm: &Relm<Self>, _: ()) -> Model {
        Model {
            relm: relm.clone(),
            days_count: 1,
        }
    }

    fn subscriptions(&mut self, _: &Relm<Self>) {
        self.reload_stats();
    }

    fn update(&mut self, event: Msg) {
        match event {
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
    }

    view! {
        #[name="window"]
        gtk::Window {
            property_default_width: 500,
            gtk::Box {
                orientation: gtk::Orientation::Vertical,
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
                    margin_bottom: 12,
                },
                #[name="most_used"]
                MostUsed {
                    CategoriesModeToggled => Msg::ToggleCategoriesMode,
                    //MostUsedMsg::ToggleCategoriesMode => Msg::ToggleCategoriesMode
                }
            },
            // Use a tuple when you want to both send a message and return a value to
            // the GTK+ callback.
            delete_event(_, _) => (Msg::Quit, Inhibit(false)),
        }
    }
}

impl Win {
    fn reload_stats(&mut self) {
        self.most_used.emit(MostUsedMsg::Clear);
        self.load_stats(self.model.days_count);
    }

    fn load_stats(&mut self, days_count: i64) {
        let frames = data::load_from_prev_days(days_count).unwrap();
        let (earliest, latest) = data::get_earliest_and_latest(&frames);

        self.total_usage
            .emit(TotalUsageMsg::SetSpan(earliest, latest));

        let entries = data::calculate_usage(frames);
        let total_usage = entries.iter().fold(0, |acc, entry| acc + entry.time);
        self.total_usage.emit(TotalUsageMsg::SetTotal(total_usage));
        self.most_used.emit(MostUsedMsg::Populate(entries));
    }
}

fn main() {
    Win::run(()).expect("Win::run failed");
}
