use std::fs;

const FILE_NAME: &str = "./input.txt";

fn is_reaction(char1: &str, char2: &str) -> bool {
    if char1 == char2 {
        return false;
    }

    if char1 == char2.to_uppercase() || char1 == char2.to_lowercase() {
        return true;
    }

    false
}

fn after_reaction(content: &str) -> String {
    let content: Vec<&str> = content.split("").collect();
    let mut new_content: Vec<&str> = Vec::new();

    for index in 0..content.len() {
        let current_char = content[index];

        match new_content.pop() {
            Some(previous_char) => {
                if is_reaction(previous_char, current_char) {
                    continue;
                }

                new_content.push(previous_char);
                new_content.push(current_char);
            }
            None => {
                new_content.push(current_char);
            }
        }
    }

    new_content.join("")
}

fn part1(content: &str) {
    let content = after_reaction(content);

    println!("remaining units size: {}", content.len());
}

fn part2(content: &str) {
    let chars: Vec<&str> = "abcdefghijklmnopqrstuvwxyz".split("").collect();
    let mut lower_length = content.len();
    let mut with_removing_char = "";

    for test in chars {
        if test.is_empty() { continue; }

        let mut new_content = String::from(content);
        new_content.retain(|x| x != test.chars().last().unwrap() && x != test.to_uppercase().chars().last().unwrap());
        let new_content = after_reaction(new_content.as_str());

        if lower_length > new_content.len() {
            lower_length = new_content.len();
            with_removing_char = test;
        }
    }

    println!("Lower length is: {} [removing: {}]", lower_length, with_removing_char);
}

fn main() {
    let content = fs::read_to_string(FILE_NAME).expect("Error while reading input file.");
    let content = content.trim();

    part1(content);
    part2(content);
}
