extern crate regex;

use std::fs;
use std::error::Error;
use std::result;
use regex::Regex;
use std::collections::HashMap;

const FILE_NAME: &str = "./input.txt";

type Result<T> = result::Result<T, Box<Error>>;

#[derive(Clone, Debug)]
struct Step {
    name: String,
    process_time: i32,
    next: Vec<String>,
    previous: Vec<String>,
}

#[derive(Clone, Debug)]
struct Worker {
    id: i32,
    step_name: Option<String>,
    available_at: i32,
}

fn part1_get_next(steps: &HashMap<String, Step>, visited: &mut Vec<String>) -> Option<Step> {
    let mut filtered_steps: Vec<&Step> = steps
        .iter()
        .map(|(_, step)| step)
        .filter(|step| step.previous.is_empty() && !visited.contains(&step.name))
        .collect();

    filtered_steps.sort_by_key(|step| &step.name);

    match filtered_steps.get(0) {
        Some(step) => {
            visited.push((*step).name.clone());
            Some((*step).clone())
        }
        None => None
    }
}

fn part1(steps: &HashMap<String, Step>) {
    let mut steps = steps.clone();

    let mut visited = Vec::<String>::new();
    while let Some(next) = part1_get_next(&steps, &mut visited) {
        print!("{}", next.name);

        for (name, step) in steps.clone().iter() {
            if step.previous.contains(&next.name) {
                steps.get_mut(name).unwrap().previous.retain(|name| name != &next.name);
            }
        }
    }

    println!("");
}

fn create_steps() -> Result<HashMap<String, Step>> {
    let content = fs::read_to_string(FILE_NAME)?;
    let re = Regex::new(r"Step (.) must be finished before step (.) can begin.*")?;

    let mut steps = HashMap::<String, Step>::new();

    for line in content.lines() {
        if line.is_empty() { continue; }

        let captures = re.captures(line).unwrap();
        let before_name = &captures[1];
        let after_name = &captures[2];

        steps.entry(String::from(before_name))
            .and_modify(|step| step.next.push(String::from(after_name)))
            .or_insert(Step {
                name: String::from(before_name),
                next: vec![String::from(after_name)],
                process_time: before_name.as_bytes()[0] as i32 - 64,
                previous: vec![],
            });

        steps.entry(String::from(after_name))
            .and_modify(|step| step.previous.push(String::from(before_name)))
            .or_insert(Step {
                name: String::from(after_name),
                next: vec![],
                process_time: after_name.as_bytes()[0] as i32 - 64,
                previous: vec![String::from(before_name)],
            });
    }

    Ok(steps)
}

fn part2(steps: &HashMap<String, Step>, workers_count: i32) {
    let mut steps = steps.clone();
    let mut workers = vec![];

    for id in 0..workers_count {
        workers.push(
            Worker {
                id: id as i32,
                step_name: None,
                available_at: 0,
            },
        );
    }

    let mut visited = vec![];
    let mut seconds = -1;

    while visited.len() < steps.len() {
        seconds += 1;

        for worker in workers.iter_mut() {
            if worker.available_at > seconds { continue; }

            if let Some(step_name) = worker.step_name.clone() {
                for (name, step) in steps.clone().iter() {
                    if step.previous.contains(&step_name) {
                        steps.get_mut(name).unwrap().previous.retain(|name| name != &step_name);
                    }
                }
            }

            if let Some(next) = part1_get_next(&steps, &mut visited) {
                worker.step_name = Some(next.name.clone());
                worker.available_at = seconds + 60 + next.process_time;
            }
        }
    }

    workers.sort_by_key(|worker| worker.available_at);
    println!("{}", workers.pop().unwrap().available_at);
}

fn main() -> Result<()> {
    let steps = create_steps()?;

    println!("--part 1--");
    part1(&steps);

    println!("--part 2--");
    part2(&steps, 5);

    Ok(())
}
