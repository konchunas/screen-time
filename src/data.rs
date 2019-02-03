use std::collections::HashMap;
use std::string::String;

use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use chrono::Local;

use std::io::{Error as IoError, ErrorKind as IoErrorKind};

pub struct Frame {
    name: String,
    start: i64,
    end: i64,
}

pub struct UsageEntry {
    pub name: String,
    pub time: i64,
}

#[derive(Debug)]
pub enum Error {
    NoFolder,
    IoError(std::io::Error),
}

static DELIM: &'static str = ";";

pub fn get_folder() -> Result<PathBuf ,Error> {
    let home_path = dirs::home_dir().unwrap();
    let folder = home_path.join(".screen-time");
    match folder.exists() {
        true => Ok(folder),
        false => Err(Error::NoFolder)
    }
}

pub fn load_collected_app_info() ->  Result<HashMap<String, String>, Error> {
    let folder_path = get_folder()?;
    let mut file = File::open(folder_path.join("app-names.csv")).map_err(Error::IoError)?;

    let mut text = String::new();
    file.read_to_string(&mut text).map_err(Error::IoError)?;

    let mut map = HashMap::new();
    for line in text.lines() {
        let words: Vec<&str> = line.split(DELIM).collect();
        if words.len() != 2 {
            eprintln!("Skipping line from desktop paths file");
            continue;
        }
        map.insert(words[0].to_string(), words[1].to_string());
    }  

    Ok(map)
}

pub fn load_from_prev_days(day_count: i64) -> Result<Vec<Frame>, Error> {
    let folder_path = get_folder()?;

    let mut frames = vec![];
    for i in 0..day_count {
        let day = Local::today() - chrono::Duration::days(i);
        let today_date = day.format("%b-%d-%Y");
        let filename = format!("{}.csv", today_date);
        let filepath = folder_path.join(filename);
        if !filepath.exists() {
            continue;
        }

        let mut day_frames = load_frames(&filepath).map_err(Error::IoError)?;
        frames.append(&mut day_frames);
    }
    return Ok(frames);
}

pub fn load_frames(path: &PathBuf) -> std::io::Result<Vec<Frame>> {
    let mut file = File::open(path)?;

    let mut text = String::new();
    file.read_to_string(&mut text)?;

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
    if frames.is_empty() {
        return (0,0);
    }

    let earliest = frames.iter().min_by_key(|frame| frame.start).unwrap();
    let latest = frames.iter().max_by_key(|frame| frame.end).unwrap();
    (earliest.start, latest.end)
}
