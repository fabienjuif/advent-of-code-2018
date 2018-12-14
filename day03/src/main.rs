extern crate regex;

use std::fs;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

const FILE_NAME: &str = "./input.txt";

fn part1() {
    let content = fs::read_to_string(FILE_NAME)
        .expect("Error while reading input file.");

    let re = Regex::new(r"#\d+ @ (\d+),(\d+): (\d+)x(\d+).*").unwrap();
    let mut map = HashMap::new();

    content.lines()
        .map(|line| {
            let cap = re.captures(line).unwrap();
            (
                cap[1].parse::<i32>().unwrap(),
                cap[2].parse::<i32>().unwrap(),
                cap[3].parse::<i32>().unwrap(),
                cap[4].parse::<i32>().unwrap()
            )
        })
        .for_each(|(x, y, w, h)| {
            for i in x..x+w {
                for j in y..y+h {
                    *map.entry((i, j)).or_insert(0) += 1;
                }
            }
        });

    let overlaps = map.values()
        .filter(|val| **val > 1)
        .count();

    println!("{}", overlaps);
}

fn part2() {
    let content = fs::read_to_string(FILE_NAME)
        .expect("Error while reading input file.");

    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+).*").unwrap();
    let mut map = HashMap::new();
    let mut ids = HashSet::new();

    content.lines()
        .map(|line| {
            let cap = re.captures(line).unwrap();
            (
                cap[1].parse::<i32>().unwrap(),
                cap[2].parse::<i32>().unwrap(),
                cap[3].parse::<i32>().unwrap(),
                cap[4].parse::<i32>().unwrap(),
                cap[5].parse::<i32>().unwrap()
            )
        })
        .for_each(|(id, x, y, w, h)| {
            ids.insert(id);

            for i in x..x+w {
                for j in y..y+h {
                    if let Some(old_id) = map.get(&(i, j)) {
                        ids.remove(old_id);
                        ids.remove(&id);
                    }

                    map.insert((i, j), id);
                }
            }
        });

    println!("{}", ids.iter().fold(-1, |_, v| *v));
}

fn main() {
    println!("-- part 1 --");
    part1();
    println!("-- part 2 --");
    part2();
}
