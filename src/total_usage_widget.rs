use relm::{Relm, Widget};
use relm_attributes::widget;

use gtk::{WidgetExt, LabelExt, OrientableExt};

use crate::time_helper::{format_duration, format_timestamp, format_datetime};

#[derive(Msg)]
pub enum Msg {
    SetSpan(i64, i64),
    SetTotal(i64),
}

pub struct Model {
    time_span: String,
    total_duration: String,
}

#[widget]
impl Widget for TotalUsage {
    fn model(_: &Relm<Self>, _: ()) -> Model {
        Model {
            time_span: " - ".to_string(),
            total_duration: "Total".to_string()
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::SetSpan(earliest, latest) => {
                let (earliest, latest) = match latest - earliest >= 86400 {
                    true => (format_datetime(earliest), format_datetime(latest)),
                    false => (format_timestamp(earliest), format_timestamp(latest)),
                };
                let time_span = format!("{}  -  {}", earliest, latest);
                self.model.time_span = time_span;
            }
            Msg::SetTotal(duration) => {
                let duration_str = format_duration(duration);
                self.model.total_duration = format!("<span size='x-large' weight='bold'>{}</span>", duration_str);
            }
        }
    }

    view! {
        gtk::Box {
            orientation: gtk::Orientation::Vertical,
            gtk::Label {
                markup: &self.model.total_duration,
                margin_top: 12,
                margin_bottom: 3,
            },
            gtk::Label {
                text: &self.model.time_span,
                margin_top: 3,
                margin_bottom: 12,
            },
        }
    }
}
