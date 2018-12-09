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

fn get_records() -> Vec<Record> {
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

    records
}

fn part1_get_most_sleep(records: &[Record]) -> String {
    let mut sleeps: HashMap<String, i64> = HashMap::new();
    let mut last_guard_id = String::from("");
    let mut last_date_time = Utc::now();

    for record in records {
        let Record { guard_id, date_time, description, .. } = record;

        if !guard_id.is_empty() { // empty guard_id means we are changing guard
            last_guard_id = guard_id.clone();
        }

        if description == "falls asleep" {
            last_date_time = date_time.clone();
        }

        if description == "wakes up" {
             let sleep = sleeps.entry(last_guard_id.clone()).or_insert(0);
            *sleep += (*date_time - last_date_time).num_minutes() + 1;
        }
    }

    let mut sorted_sleeps = Vec::new();
    for (guard_id, sleep) in sleeps {
        sorted_sleeps.push((guard_id, sleep));
    }
    sorted_sleeps.sort_by(|sleep_a, sleep_b| sleep_b.1.cmp(&sleep_a.1));

    let (guard_id, _) = sorted_sleeps.get(0).unwrap();

    guard_id.to_string()
}

fn part1_get_most_seen_minute(records: &[Record], most_sleep_guard_id: String) -> i64 {
    println!("must sleep: #{}", most_sleep_guard_id);

    let mut last_guard_id = String::from("");
    let mut last_date_time = Utc::now();
    let mut last_minute = String::from("");
    let mut last_fulldate = String::from("");
    let mut minutes = HashMap::new();
    let mut sorted_minutes = Vec::new();

    for record in records {
        let Record { guard_id, fulldate, date_time, description, minute, .. } = record;

        if !guard_id.is_empty() { // empty guard_id means we are changing guard
            last_guard_id = guard_id.clone();
        }

        if last_guard_id != most_sleep_guard_id {
            continue;
        }

        if description == "falls asleep" {
            last_date_time = date_time.clone();
            last_minute = minute.clone();
            last_fulldate = fulldate.clone();
        }

        if description == "wakes up" {
            let num_minutes = (*date_time - last_date_time).num_minutes();

            for min in 0..num_minutes {
                *minutes.entry(last_minute.parse::<i64>().unwrap() + min).or_insert(0) += 1;
            }
        }
    }

    for (minute, times) in minutes {
        sorted_minutes.push((minute, times));
    }
    sorted_minutes.sort_by(|minute_a, minute_b| minute_b.1.cmp(&minute_a.1));

    sorted_minutes.get(0).unwrap().0
}

fn part1(records: &[Record]) {
    let most_sleep_guard_id = part1_get_most_sleep(records);
    let most_seen_minute = part1_get_most_seen_minute(records, most_sleep_guard_id.clone());

    println!(
        "#{} - most seen asleep: {} --> {}x{} => {}",
        most_sleep_guard_id,
        most_seen_minute,
        most_sleep_guard_id,
        most_seen_minute,
        most_sleep_guard_id.parse::<i64>().unwrap() * most_seen_minute,
    )
}

fn main() {
    let records = get_records();
    println!("records size: {}", records.len());

    println!("-- part1 --");
    part1(records.as_slice());
}
