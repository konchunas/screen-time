use std::collections::HashMap;
use std::string::String;

use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use chrono::Local;

use std::io::{Error, ErrorKind};

pub struct Frame {
    name: String,
    start: i64,
    end: i64,
}

pub struct UsageEntry {
    pub name: String,
    pub time: i64,
}

static DELIM: &'static str = ";";

pub fn load_from_prev_days(day_count: i64) -> std::io::Result<Vec<Frame>> {
    let mut folder_path = dirs::home_dir().unwrap();
    folder_path.push(".screen-time");
    if !folder_path.exists() {
        return Err(Error::new(
            ErrorKind::Other,
            "No screen time logs exist. Is screen time daemon installed?",
        ));
    }

    let mut frames = vec![];
    for i in 0..day_count {
        let day = Local::today() - chrono::Duration::days(i);
        let today_date = day.format("%b-%d-%Y");
        let filename = format!("{}.csv", today_date);
        let filepath = folder_path.join(filename);
        if !filepath.exists() {
            continue;
        }

        let mut day_frames = load_frames(&filepath)?;
        frames.append(&mut day_frames);
    }
    return Ok(frames);
}

pub fn load_frames(path: &PathBuf) -> std::io::Result<Vec<Frame>> {
    let mut file = File::open(path).unwrap();

    let mut text = String::new();
    file.read_to_string(&mut text)
        .expect("Cannot read Screen Time log for today");

    let mut frames = Vec::new();
    for line in text.lines() {
        //  counter += 1;
        let mut words = line.split(DELIM);
        let frame: Result<Frame, bool> = (|| {
            Ok(Frame {
                name: words.next().ok_or(false)?.into(),
                start: words
                    .next()
                    .ok_or(false)?
                    .parse::<i64>()
                    .map_err(|_| false)?,
                end: words
                    .next()
                    .ok_or(false)?
                    .parse::<i64>()
                    .map_err(|_| false)?,
            })
        })();
        match frame {
            Ok(frame) => {
                // let duration = frame.end - frame.start;
                // println!(
                //     "frame duration {} at line {}",
                //     crate::time_helper::format_duration(duration),
                //     counter
                // );
                // println!(
                //     "from {} to {}",
                //     crate::time_helper::format_timestamp(frame.start),
                //     crate::time_helper::format_timestamp(frame.end)
                // );
                frames.push(frame)
            }
            Err(_) => eprintln!("Skipping reading line"),
        }
    }
    Ok(frames)
}

pub fn calculate_usage(frames: Vec<Frame>) -> Vec<UsageEntry> {
    let mut map: HashMap<String, i64> = HashMap::new();
    // print("frames count: %u\n", frames.length);
    for frame in frames {
        let frame_time = frame.end - frame.start;
        *map.entry(frame.name).or_insert(0) += frame_time;

        // if !map.contains_key(&frame.name) {
        //     map[frame.name] = frame_time;
        // } else {
        //     map[frame.name] += frame_time;
        // }
    }

    let mut entries: Vec<UsageEntry> = map
        .into_iter()
        .map(|(name, time)| UsageEntry { name, time })
        .collect();

    entries.sort_unstable_by(|a, b| b.time.cmp(&a.time));
    return entries;
}

pub fn get_earliest_and_latest(frames: &Vec<Frame>) -> (i64, i64) {
    let earliest = frames.iter().min_by_key(|frame| frame.start).unwrap();
    let latest = frames.iter().max_by_key(|frame| frame.end).unwrap();
    (earliest.start, latest.end)
}
