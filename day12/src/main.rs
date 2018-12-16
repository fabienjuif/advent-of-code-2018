use std::fs;
use std::result;
use std::error::Error;
use std::boxed::Box;

const FILE_NAME: &str = "./input.txt";

type Result<T> = result::Result<T, Box<Error>>;

fn print_score(pots: String, patterns: &[(&str, &str)], generations: i64) {
    let mut pots = String::from(pots);
    let mut new_pots = String::from(pots.clone());

    let mut negative_offset = 0;
    for generation in 0..generations {
        // pushing values so pattern will work
        while !pots.starts_with(".....") {
            pots.insert_str(0, ".");
            new_pots.insert_str(0, ".");
            negative_offset += 1;
        }
        while !pots.ends_with(".....") {
            pots.push_str(".");
            new_pots.push_str(".");
        }

        unsafe {
            for b in new_pots.as_mut_vec() {
                *b = '.' as u8;
            }
        }

        if generation % 500 == 0 {
            println!("iters: {:?}", patterns.len() * (pots.len() - 5));
        }

        for pattern in patterns {
            for index in 0..pots.len()-5 {
                if pots[index..index+5] == *pattern.0 {
                    unsafe {
                        new_pots.as_mut_vec()[index + 2] = '#' as u8;
                    }
                }
            }
            // this was the older version
            // but when I read the documenation (and read documentaiton because I was failing...)
            // I saw that .match_indices doesn't handle overlaps
            // So I made the previous version to handle that on my own
            // ------------
            // for (index, m) in pots.match_indices(pattern.0) {
            //     println!("m: ({} -> {})", index, m);
            //     new_pots.replace_range(index+2..index+3, pattern.1);
            // }
            // ------------
        }

        let buffer = pots;
        pots = new_pots;
        new_pots = buffer;
    }

    let mut score = 0;
    for (index, pot) in pots.chars().enumerate() {
        if pot == '#' {
            score += index as i64 - negative_offset;
        }
    }

    println!("{}", score);
}

fn main() -> Result<()> {
    let content = fs::read_to_string(FILE_NAME)?;
    let mut pots = "";
    let mut patterns = vec![];
    for (index, line) in content.lines().enumerate() {
        if index == 0 {
            pots = line.split("initial state: ").nth(1).unwrap();
        } else if !line.is_empty() {
            let mut s = line.split(" => ");
            let left = s.next().unwrap();
            let right = s.next().unwrap();

            if right == "#" {
                patterns.push((
                    left,
                    right,
                ));
            }
        }
    }

    println!("--part 1--");
    print_score(String::from(pots), &patterns, 20);

    println!("--part 2--");
    print!("500 gens: ");
    print_score(String::from(pots), &patterns, 500);
    print!("5_000 gens: ");
    print_score(String::from(pots), &patterns, 5_000);
    print!("50_000 gens: ");
    print_score(String::from(pots), &patterns, 50_000);
    // you should see that: https://github.com/BurntSushi/advent-of-code/blob/master/aoc12/src/main.rs#L27

    // print_score(String::from(pots), &patterns, 50_000_000_000);
    // print_score(String::from(pots), &patterns, 50000000000);
    // print_score(String::from(pots), &patterns, 50000000000);

    Ok(())
}
