use std::fs;
use std::collections::HashMap;

const FILE_NAME: &str = "./input.txt";

#[derive(Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
    owner_id: i32,
    distance_from_owner: i32,
    visited: bool,
}

impl Point {
    fn distance_to(&self, point: &Point) -> i32 {
        (self.x - point.x).abs() + (self.y - point.y).abs()
    }
}

fn main() {
    let content = fs::read_to_string(FILE_NAME).expect("Error reading input file.");
    let mut points = Vec::<Point>::new();

    let mut bound = Point { x: 0, y: 0, owner_id: -1, distance_from_owner: -1, visited: false };

    for (owner_id, line) in content.lines().enumerate() {
        let coords: Vec<&str> = line.split(", ").collect();

        let point = Point {
            owner_id: owner_id as i32,
            visited: true,
            distance_from_owner: 0, // this is the owner
            x: coords.get(0).unwrap().parse::<i32>().unwrap(),
            y: coords.get(1).unwrap().parse::<i32>().unwrap(),
        };

        if point.x > bound.x {
            bound.x = point.x.clone();
        }

        if point.y > bound.y {
            bound.y = point.y.clone();
        }

        points.push(point);
    }

    let mut grid = Vec::<Point>::new();
    let mut owner_to_exclude = Vec::<i32>::new();
    for x in 0..=bound.x {
        for y in 0..=bound.y {
            let mut new_owned_point = Point {
                x,
                y,
                owner_id: -1,
                visited: false,
                distance_from_owner: -1,
            };

            for point in points.as_slice() {
                let distance = point.distance_to(&new_owned_point);

                if distance == new_owned_point.distance_from_owner {
                    new_owned_point.owner_id = -1;
                } else if new_owned_point.distance_from_owner == -1
                    || distance < new_owned_point.distance_from_owner
                {
                    new_owned_point.distance_from_owner = distance;
                    new_owned_point.owner_id = point.owner_id;
                }
            }

            new_owned_point.visited = true;

            if new_owned_point.x == 0
                || new_owned_point.y == 0
                || new_owned_point.x == bound.x
                || new_owned_point.y == bound.y
            {
                owner_to_exclude.push(new_owned_point.owner_id);
            }

            grid.push(new_owned_point);
        }
    }

    let mut nb_points_by_owner = HashMap::<i32, i32>::new();
    for point in grid.as_slice() {
        if point.owner_id != -1 && !owner_to_exclude.contains(&point.owner_id) {
            *nb_points_by_owner.entry(point.owner_id).or_insert(0) += 1;
        }
    }

    let mut max = 0;
    for (_, value) in nb_points_by_owner {
        if value > max {
            max = value;
        }
    }

    println!("max: {}", max);

    // let mut sorted_grid = grid.to_vec();
    // sorted_grid.sort_by_key(|point| (-point.y, point.x));

    // println!("<div>");
    // for (index, point) in sorted_grid.iter().enumerate() {
    //     if index as f32 % (bound.x + 1) as f32 == 0.0 {
    //         println!("</div><div class=\"line\">");
    //     }

    //     print!("\t<div class=\"cell cell-{} ", index);
    //     if point.distance_from_owner == 0 {
    //         print!("owner\">{}", point.owner_id);
    //     } else if point.owner_id == -1 {
    //         print!("nobody\">");
    //     } else {
    //         print!("owned\">{}", point.owner_id);
    //     }
    //     println!("</div>");
    // }

    // println!("</div>");
}
