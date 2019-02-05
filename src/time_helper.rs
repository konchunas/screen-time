use chrono::{NaiveDateTime, Datelike, Timelike, Local, TimeZone, Duration};

pub fn format_duration(time: i64) -> String {
    let duration = Duration::seconds(time);
    let hours = duration.num_hours();
    let minutes_duration = duration - Duration::hours(hours);
    let minutes = minutes_duration.num_minutes();
    let seconds_duration = minutes_duration - Duration::minutes(minutes);
    let seconds = seconds_duration.num_seconds();

    let time_str = match hours > 0 {
        true => format!("{}h {}m", hours, minutes),
        false => match  minutes > 0 {
            true => format!("{}m", minutes),
            false => format!("{}s", seconds)
        }
    };

    return time_str;
}

pub fn format_timestamp(time: i64) -> String {
    let date_time = Local.timestamp(time, 0);
    return date_time.time().format("%R").to_string(); //TODO preffered system format
}

pub fn format_datetime(time: i64) -> String {
    let date_time = Local.timestamp(time, 0);
    return date_time.format("%-d %b %R").to_string(); //TODO preffered system format
}