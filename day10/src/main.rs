use std::fs;
use std::result;
use std::error::Error;
use regex::Regex;
use std::collections::HashSet;

const FILE_NAME:&str = "./input.txt";

type Result<T> = result::Result<T, Box<Error>>;

fn get_bounds(points: &[(usize, (i32, i32), (i32, i32))]) -> ((i32, i32), (i32, i32)) {
    let mut max_bound = (0, 0);
    let mut min_bound = (0, 0);

    for point in points {
        if max_bound.0 < (point.1).0 {
            max_bound.0 = (point.1).0;
        }
        if max_bound.1 < (point.1).1 {
            max_bound.1 = (point.1).1;
        }
        if min_bound.0 > (point.1).0 {
            min_bound.0 = (point.1).0;
        }
        if min_bound.1 > (point.1).1 {
            min_bound.1 = (point.1).1;
        }
    }

    (min_bound, max_bound)
}

fn print(points: &[(usize, (i32, i32), (i32, i32))], bounds: ((i32, i32), (i32, i32))) {
    let (min_bound, max_bound) = bounds;

    let known_points = points.iter().map(|point| point.1).collect::<HashSet<_>>();
    for y in min_bound.1..=max_bound.1 {
        for x in min_bound.0..=max_bound.0 {
            match known_points.get(&(x, y)) {
                None => print!("."),
                Some(_) => print!("#"),
            }
        }
        println!("");
    }
}

fn main() -> Result<()> {
    let content = fs::read_to_string(FILE_NAME)?;
    let re = Regex::new(r"position=<(.*?),(.*?)> velocity=<(.*?),(.*?)>.*")?;

    let mut points = vec![];
    for (index, line) in content.lines().enumerate() {
        if let Some(captures) = re.captures(line) {
            let next_point = (
                index,
                (
                    captures[1].trim().parse::<i32>()?,
                    captures[2].trim().parse::<i32>()?,
                ),
                (
                    captures[3].trim().parse::<i32>()?,
                    captures[4].trim().parse::<i32>()?,
                )
            );

            points.push(next_point);
        }
    }

    let mut last_size = 0;
    let mut time = 0;
    loop {
        time += 1;

        for mut point in points.as_mut_slice() {
            (point.1).0 += (point.2).0;
            (point.1).1 += (point.2).1;
        }

        let bounds = get_bounds(points.as_slice());
        let (min_bound, max_bound) = bounds;
        let size = (min_bound.0.abs() as u64 + max_bound.0.abs() as u64) * (min_bound.1.abs() as u64 + max_bound.1.abs() as u64);

        if last_size != 0 && last_size < size {
            println!("Done at {} secs, reverting back", time - 1);

            for mut point in points.as_mut_slice() {
                (point.1).0 -= (point.2).0;
                (point.1).1 -= (point.2).1;
            }

            print(points.as_slice(), bounds);

            break;
        }

        last_size = size;
    }

    Ok(())
}
