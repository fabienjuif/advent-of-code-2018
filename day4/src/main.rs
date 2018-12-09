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
    minute: String,
    description: String,
}

#[derive(Clone, Debug)]
struct Guard {
    id: String,
    minutes: HashMap<i64, i64>,
    sorted_minutes: Vec<(i64, i64)>,
    most_minute_asleep: (i64, i64),
    total_asleep: i64,
}

fn get_guards() -> Vec<Guard> {
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
    let mut last_date_time = Utc::now();
    let mut last_minute = String::from("");
    let mut last_guard_id = String::from("");

    for record in records {
        let Record { guard_id, date_time, description, minute, .. } = record;

        last_guard_id = if guard_id.is_empty() { last_guard_id } else { guard_id.clone() };

        let mut current_guard = guards.entry(last_guard_id.clone()).or_insert(Guard {
            id: guard_id.clone(),
            minutes: HashMap::new(),
            sorted_minutes: Vec::new(),
            most_minute_asleep: (0, 0),
            total_asleep: 0,
        });

        if !guard_id.is_empty() { // empty guard_id means we are changing guard
            for (minute, times) in current_guard.minutes.iter() {
                current_guard.sorted_minutes.push((minute.clone(), times.clone()));
            }
            current_guard.sorted_minutes.sort_by(|minute_a, minute_b| minute_b.1.cmp(&minute_a.1));

            current_guard.most_minute_asleep = match current_guard.sorted_minutes.get(0) {
                Some(value) => value.clone(),
                None => (0, 0)
            };
        }

        if description == "falls asleep" {
            last_date_time = date_time.clone();
            last_minute = minute.clone();
        }

        if description == "wakes up" {
            current_guard.total_asleep += (date_time - last_date_time).num_minutes() + 1;

            let num_minutes = (date_time - last_date_time).num_minutes();

            for min in 0..num_minutes {
                *current_guard.minutes.entry(last_minute.parse::<i64>().unwrap() + min).or_insert(0) += 1;
            }
        }
    }


    let mut guard_values = Vec::new();

    for (_, value) in guards {
        guard_values.push(value);
    }

    guard_values
}

fn part1(mut guards: Vec<Guard>) {
    guards.sort_by(|guard_a, guard_b| guard_b.total_asleep.cmp(&guard_a.total_asleep));
    let guard = guards.get(0).unwrap();
    println!(
        "#{} - {} --> {}",
        guard.id,
        guard.most_minute_asleep.0,
        guard.id.parse::<i64>().unwrap() * guard.most_minute_asleep.0,
    );
}

fn part2(mut guards: Vec<Guard>) {
    guards.sort_by(|guard_a, guard_b| guard_b.most_minute_asleep.1.cmp(&guard_a.most_minute_asleep.1));
    let guard = guards.get(0).unwrap();
    println!(
        "#{} - {} --> {}",
        guard.id,
        guard.most_minute_asleep.0,
        guard.id.parse::<i64>().unwrap() * guard.most_minute_asleep.0,
    );
}


fn main() {
    let guards = get_guards();
    println!("guards size: {}", guards.len());

    println!("-- part1 --");
    part1(guards.to_vec());

    println!("-- part2 --");
    part2(guards.to_vec());
}
