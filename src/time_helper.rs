use chrono::{NaiveDateTime, Timelike, DateTime, Local, TimeZone};

pub fn format_duration(time: i64) -> String {
    let date_time = NaiveDateTime::from_timestamp(time, 0);
    let time = date_time.time();
    let time_str = match time.hour() > 0 {
        true => time.format("%_Hh %_Mm").to_string(),
        false => match time.minute() > 0 {
            true => time.format("%_Mm").to_string(),
            false => time.format("%_Ss").to_string(),
        }
    };

    return time_str;
}

pub fn format_timestamp(time: i64) -> String {
    let date_time = Local.timestamp(time, 0);
    return date_time.time().format("%R").to_string(); //TODO preffered system format
}