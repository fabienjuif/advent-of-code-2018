use std::collections::HashSet;
use std::fs;

const FILE_NAME: &str = "./input.txt";

fn part1() {
    let content = fs::read_to_string(FILE_NAME)
        .expect("Error while reading input file.");

    let mut duos_counter = 0;
    let mut trios_counter = 0;

    for line in content.lines() {
        if line.is_empty() { continue; }

        let mut seen: HashSet<&str> = HashSet::new();
        let mut duos: HashSet<&str> = HashSet::new();
        let mut trios: HashSet<&str> = HashSet::new();

        for c in line.split("") {
            if c == "" { continue; }

            if seen.contains(c) {
                if duos.contains(c) {
                    trios.insert(c);
                }
                duos.insert(c);
            }
            seen.insert(c);
        }

        for c in trios.iter() {
            duos.remove(c);
        }

        if !duos.is_empty() {
            duos_counter += 1;
        }

        if !trios.is_empty() {
            trios_counter += 1;
        }
    }

    println!("Duos: {} | Trios: {}", duos_counter, trios_counter);
    println!("checksum: {}", duos_counter * trios_counter);
}

fn part2_find_lines () -> (String, String) {
    let content = fs::read_to_string(FILE_NAME)
        .expect("Error while reading input file");

    let lines: Vec<&str> = content.lines().collect();

    for idx_line_a in 0..lines.len() {
        let line_a = lines[idx_line_a];

        for idx_line_b in idx_line_a..lines.len() {
            if idx_line_b + 1 == lines.len() { break; }
            let line_b = lines[idx_line_b + 1];

            let mut diffs = 0;
            for idx_char in 0..line_a.len() {
                if line_a.chars().nth(idx_char) != line_b.chars().nth(idx_char) {
                    diffs += 1;
                }

                if diffs > 1 { break; }
            }

            if diffs <= 1 {
                return (line_a.to_string(), line_b.to_string());
            }
        }
    }

    return (String::from("nothing"), String::from("nothing"));
}

fn part2 () {
    let (line_a, line_b) = part2_find_lines();

    let mut trimed_str = String::new();
    let chars_a: Vec<char> = line_a.chars().collect();
    let chars_b: Vec<char> = line_b.chars().collect();

    for i in 0..chars_a.len() {
        if chars_a[i] == chars_b[i] {
            trimed_str.push(chars_a[i]);
        }
    }

    println!("{}  --  {}", line_a, line_b);
    println!("{}", trimed_str);
}

fn main () {
    println!("--part1--");
    part1();

    println!("--part2--");
    part2();
}
