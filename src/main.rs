extern crate gtk;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;
extern crate relm_attributes;

use relm_attributes::widget;

use gtk::{
    BoxExt,
    Button,
    ButtonExt,
    ButtonBoxExt,
    RadioButtonExt,
    ContainerExt,
    Inhibit,
    FrameExt,
    Label,
    LabelExt,
    OrientableExt,
    ToggleButtonExt,
    WidgetExt,
    Window,
    WindowType,

};
use gtk::Orientation::*;
use relm::{Relm, Update, Widget};
use relm::{Component, ContainerWidget};

mod usage_widget;
mod data;
mod time_helper;

use crate::usage_widget::UsageWidget;
use crate::usage_widget::Model as AppUsageModel;


#[derive(Msg)]
pub enum Msg {
    Add(String, i64, f64),
    SetEarliestAndLatest(i64, i64),
    ShowWeekStats,
    ShowDayStats,
    Quit,
}

pub struct Model {
    relm: Relm<Win>,
    usage_widgets: Vec<Component<UsageWidget>>
}

#[widget]
impl Widget for Win {
    fn model(relm: &Relm<Self>, _: ()) -> Model {
        Model {
            relm: relm.clone(),
            usage_widgets: vec![]
        }
    }

    fn subscriptions(&mut self, _: &Relm<Self>) {
        self.load_stats(1);
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Add(name, duration, fraction) => {
                let widget = self.most_used.add_widget::<UsageWidget>((name, duration, fraction));
                self.model.usage_widgets.push(widget);
            },
            Msg::ShowDayStats => {
                if self.today_radio.get_active() {
                    while let Some(widget) = self.model.usage_widgets.pop() {
                        self.most_used.remove_widget(widget);
                    }
                    self.load_stats(1);
                }
            },
            Msg::ShowWeekStats => {
                if self.week_radio.get_active() {
                    while let Some(widget) = self.model.usage_widgets.pop() {
                        self.most_used.remove_widget(widget);
                    }
                    self.load_stats(7);
                }
            },
            Msg::SetEarliestAndLatest(_, _) => {
                //TODO
            }
            Msg::Quit => gtk::main_quit(),
        }
    }

    fn init_view(&mut self) {
        self.week_radio.join_group(Some(&self.today_radio));
    }

    view! {
        gtk::Window {
            gtk::Box {
                orientation: gtk::Orientation::Vertical,
                spacing: 12,
                gtk::ButtonBox {
                    orientation: gtk::Orientation::Horizontal,
                    layout: gtk::ButtonBoxStyle::Expand,
                    halign: ::gtk::Align::Center,
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

                gtk::Frame {
                    label: "Most used",
                    #[name="most_used"]
                    gtk::Box {
                        orientation: Vertical,
                    },

                    child: {
                        padding: 6
                    }
                }
            },
            // Use a tuple when you want to both send a message and return a value to
            // the GTK+ callback.
            delete_event(_, _) => (Msg::Quit, Inhibit(false)),
        }
    }
}

impl Win {
    fn load_stats(&self, days_count: i64) {
        let frames = data::load_from_prev_days(days_count).unwrap();   
        let (earliest, latest) = data::get_earliest_and_latest(&frames);

        self.model.relm.stream().emit(Msg::SetEarliestAndLatest(earliest, latest));

        let entries = data::calculate_usage(frames);
        let total_usage = entries.iter().fold(0, |acc, entry| acc + entry.time);
        for entry in entries {
            let fraction = entry.time as f64 / total_usage as f64;
            self.model.relm.stream().emit(Msg::Add(entry.name, entry.time, fraction));
        }
    }
}

fn main() {
    Win::run(()).expect("Win::run failed");
    
}