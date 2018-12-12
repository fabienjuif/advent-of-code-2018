extern crate regex;

use std::fs;
use std::error::Error;
use std::result;
use regex::Regex;
use std::collections::HashMap;
use std::option::Option;

const FILE_NAME: &str = "./input.txt";

type Result<T> = result::Result<T, Box<Error>>;

#[derive(Clone, Debug)]
struct Step {
    name: String,
    next: Vec<String>,
    previous: Vec<String>,
}

fn get_next(steps: &HashMap<String, Step>, visited: &Vec<String>) -> Option<Step> {
    let mut filtered_steps: Vec<&Step> = steps
        .iter()
        .map(|(_, step)| step)
        .filter(|step| step.previous.is_empty() && !visited.contains(&step.name))
        .collect();

    filtered_steps.sort_by_key(|step| &step.name);

    match filtered_steps.get(0) {
        Some(step) => Option::Some((*step).clone()),
        None => Option::None
    }
}

fn main() -> Result<()> {
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
                previous: vec![],
            });

        steps.entry(String::from(after_name))
            .and_modify(|step| step.previous.push(String::from(before_name)))
            .or_insert(Step {
                name: String::from(after_name),
                next: vec![],
                previous: vec![String::from(before_name)],
            });
    }

    let mut visited = Vec::<String>::new();
    while let Some(next) = get_next(&steps, &visited) {
        visited.push(next.name.clone());
        print!("{}", next.name);

        for (name, step) in steps.clone().iter() {
            if step.previous.contains(&next.name) {
                steps.get_mut(name).unwrap().previous.retain(|name| name != &next.name);
            }
        }
    }

    println!("");

    Ok(())
}
