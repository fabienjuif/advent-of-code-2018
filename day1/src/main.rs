use std::collections::HashSet;
use std::fs;

const FILE_NAME: &str = "./input.txt";

fn functionnal_part1 () {
    let content = fs::read_to_string(FILE_NAME)
        .expect("Error while reading the input file");

    let content = content.split("\n");

    let count:i32 = content
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<i32>().expect("Error while parsing line"))
        .sum();

    println!("{}", count);
}

fn part2() {
    let content = fs::read_to_string(FILE_NAME)
        .expect("Error while reading the input file");

    let mut previous = HashSet::new();
    let mut count:i32 = 0;

    previous.insert(count);

    loop {
        for line in content.lines() {
            if !line.is_empty() {
                let current: i32 = line.parse()
                    .expect("Error while parsing line");

                count += current;

                if previous.contains(&count) {
                    println!("{}", count);
                    return;
                }

                previous.insert(count);
            }
        }
    }
}

fn main() {
    functionnal_part1();
    part2();
}
