extern crate chrono;
extern crate regex;

use std::fs;
use regex::Regex;
use std::collections::HashMap;
use chrono::prelude::*;

const FILE_NAME: &str = "./input.txt";

struct Record {
    guard_id: String,
    date_time: DateTime<Utc>,
    fulldate: String,
    // date: String,
    // hour: String,
    minute: String,
    description: String,
}

struct Guard {
    id: String,
    minutes: Vec<(i64, i64)>,
    most_minute_asleep: (i64, i64),
    total_asleep: i64,
}

fn get_guards() {
    let content = fs::read_to_string(FILE_NAME).unwrap();
    let re = Regex::new(r"\[(\d+-\d+-\d+) (\d+):(\d+)\]( Guard #)?(\d+)? (.*)").unwrap();
    let mut records: Vec<Record> = Vec::new();

    for line in content.lines() {
        match re.captures(line) {
            Some(captures) => {
                let date = captures.get(1).unwrap().as_str().to_string();
                let hour = captures.get(2).unwrap().as_str().to_string();
                let minute = captures.get(3).unwrap().as_str().to_string();
                let fulldate = format!("{}T{}:{}:00Z", date, hour, minute);
                let description = captures.get(6).unwrap().as_str().to_string();
                let guard_id = match captures.get(5) {
                    Some(matcher) => matcher.as_str().to_string(),
                    None => String::from("")
                };
                let date_time = fulldate.parse::<DateTime<Utc>>().unwrap();

                records.push(Record {
                    guard_id,
                    date_time,
                    fulldate,
                    // date,
                    // hour,
                    minute,
                    description,
                });
            },
            None => {}
        }
    }

    records.sort_by(|record_a, record_b| record_a.date_time.cmp(&record_b.date_time));
    println!("records size: {}", records.len());

    let mut guards: HashMap<String, Guard> = HashMap::new();
    let mut current_guard: &mut Guard = &mut Guard { id: String::from(""), minutes: Vec::new(), most_minute_asleep: (0, 0), total_asleep: 0 };
    let mut last_date_time = Utc::now();
    let mut last_minute = String::from("");
    let mut minutes: HashMap<i64, i64> = HashMap::new();
    let mut sorted_minutes = Vec::new();

    for record in records {
        let Record { guard_id, date_time, description, minute, .. } = record;

        if !guard_id.is_empty() { // empty guard_id means we are changing guard
            current_guard = guards.entry(guard_id.clone()).or_insert(Guard {
                id: guard_id.clone(),
                minutes: Vec::new(),
                most_minute_asleep: (0, 0),
                total_asleep: 0,
            });

            for (minute, times) in minutes.iter() {
                sorted_minutes.push((minute.clone(), times.clone()));
            }
            sorted_minutes.sort_by(|minute_a, minute_b| minute_b.1.cmp(&minute_a.1));

            current_guard.minutes = sorted_minutes.drain(..).collect();
            current_guard.most_minute_asleep = match current_guard.minutes.get(0) {
                Some(value) => value.clone(),
                None => (0, 0)
            };

            minutes.clear();
            sorted_minutes.clear();
        }

        if description == "falls asleep" {
            last_date_time = date_time.clone();
            last_minute = minute.clone();
        }

        if description == "wakes up" {
            current_guard.total_asleep += (date_time - last_date_time).num_minutes() + 1;

            let num_minutes = (date_time - last_date_time).num_minutes();

            for min in 0..num_minutes {
                *minutes.entry(last_minute.parse::<i64>().unwrap() + min).or_insert(0) += 1;
            }
        }
    }
}

fn main() {
    get_guards();
}
