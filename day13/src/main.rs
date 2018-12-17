use std::fs;
use std::result;
use std::error::Error;
use std::boxed::Box;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

const FILE_NAME:&str = "./input.txt";
const DEBUG:bool = false;

type Result<T> = result::Result<T, Box<Error>>;

#[derive(Debug, PartialEq, Clone)]
enum Capability {
    Vertical,
    Horizontal,
    All,
    None,
}

impl Default for Capability {
    fn default() -> Capability {
        Capability::None
    }
}

#[derive(Debug, Default, Clone)]
struct Point {
    x: usize,
    y: usize,
    capability: Capability,
    intersection: bool,
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x
        && self.y == other.y
    }
}

impl Eq for Point {}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl Point {
    fn from_position(x: usize, y: usize) -> Point {
        Point {
            x,
            y,
            ..Default::default()
        }
    }

    fn from(x: usize, y: usize, point_type: char) -> Option<Point> {
        let capability = match point_type {
            '-' => Capability::Horizontal,
            '|' => Capability::Vertical,
            '/' => Capability::All,
            '\\' => Capability::All,
            '>' => Capability::Horizontal,
            '<' => Capability::Horizontal,
            '^' => Capability::Vertical,
            'v' => Capability::Vertical,
            '+' => Capability::All,
            _ => {
                return None;
            },
        };

        let intersection = match point_type {
            '+' => true,
            _ => false,
        };

        Some(Point {
            x,
            y,
            capability,
            intersection,
        })
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Top,
    Bottom,
    None,
}

impl Default for Direction {
    fn default() -> Direction {
        Direction::None
    }
}

#[derive(Debug, Default)]
struct Kart {
    point: Point,
    direction: Direction,
    cycle: usize,
}

impl PartialEq for Kart {
    fn eq(&self, other: &Kart) -> bool {
        self.point == other.point
    }
}

impl Eq for Kart {}

impl Hash for Kart {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.point.hash(state);
    }
}

impl Kart {
    fn from(x: usize, y: usize, point_type: char) -> Option<Kart> {
        let direction = match point_type {
            '>' => Direction::Right,
            '<' => Direction::Left,
            '^' => Direction::Top,
            'v' => Direction::Bottom,
            _ => {
                return None;
            }
        };

        Some(Kart {
            point: Point {
                x,
                y,
                ..Default::default()
            },
            direction,
            cycle: 2,
        })
    }

    fn from_position(x: usize, y: usize) -> Kart {
        Kart {
            point: Point {
                x,
                y,
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

#[derive(Default, Debug)]
struct Grid {
    bound: Point,
    points: HashSet<Point>,
    karts: Vec<Kart>,
}

impl Grid {
    fn new() -> Grid {
        Grid {
            ..Default::default()
        }
    }

    fn add_point(&mut self, point: Point) {
        if self.bound.x < point.x {
            self.bound.x = point.x;
        }
        if self.bound.y < point.y {
            self.bound.y = point.y;
        }

        self.points.insert(point);
    }

    fn add_kart(&mut self, kart: Kart) {
        self.karts.push(kart);
    }

    fn remove_karts_from_position(&mut self, point: &Point) -> usize {
        while let Some(index) = self.karts.iter().position(|kart| kart.point == *point) {
            if DEBUG { println!("Removing karts at ({},{})", point.x, point.y); }
            self.karts.remove(index);
        }

        self.karts.len()
    }

    fn step(&mut self) -> Vec<Point> {
        for kart in self.karts.as_mut_slice() {
            // move the kart
            match &kart.direction {
                Direction::Right => kart.point.x += 1,
                Direction::Left => kart.point.x -= 1,
                Direction::Top => kart.point.y -= 1,
                Direction::Bottom => kart.point.y += 1,
                _ => panic!("unknown direction"),
            }

            match self.points.get(&kart.point) {
                Some(point) => {
                    match point.intersection {
                        true => {
                            if kart.cycle == 2 { kart.cycle = 0; }
                            else { kart.cycle += 1; }

                            match kart.cycle {
                                0 => {
                                    kart.direction = match kart.direction {
                                        Direction::Left => Direction::Bottom,
                                        Direction::Right => Direction::Top,
                                        Direction::Bottom => Direction::Right,
                                        Direction::Top => Direction::Left,
                                        _ => panic!("Unknown direction"),
                                    };
                                },
                                1 => {},
                                2 => {
                                    kart.direction = match kart.direction {
                                        Direction::Left => Direction::Top,
                                        Direction::Right => Direction::Bottom,
                                        Direction::Bottom => Direction::Left,
                                        Direction::Top => Direction::Right,
                                        _ => panic!("Unknown direction"),
                                    };
                                },
                                _ => panic!("Unknown cycle")
                            };
                        },
                        false => {
                            if point.capability == Capability::All {
                                kart.direction = match kart.direction {
                                    Direction::Right | Direction::Left => match self.points.get(&Point::from_position(kart.point.x, kart.point.y + 1)) {
                                        Some(next_point) => match next_point.capability {
                                            Capability::Vertical => Direction::Bottom,
                                            Capability::All => match next_point.intersection {
                                                true => Direction::Bottom,
                                                false => Direction::Top,
                                            },
                                            _ => Direction::Top,
                                        },
                                        None => Direction::Top,
                                    },
                                    Direction::Top | Direction::Bottom => match self.points.get(&Point::from_position(kart.point.x + 1, kart.point.y)) {
                                        Some(next_point) => match next_point.capability {
                                            Capability::Horizontal => Direction::Right,
                                            Capability::All => match next_point.intersection {
                                                true => Direction::Right,
                                                false => Direction::Left,
                                            },
                                            _ => Direction::Left
                                        },
                                        None => Direction::Left,
                                    },
                                    _ => panic!("kart should not be in this unknown direction!")
                                };
                            }
                        }
                    }
                },
                None => panic!("point not found in grid: {:?}", kart.point)
            }
        }

        if DEBUG {
            self.print();
        }

        let mut collisions = vec![];
        for (index, kart) in self.karts.iter().enumerate() {
            for other_kart in self.karts.iter().skip(index + 1) {
                if kart.point == other_kart.point {
                    collisions.push(kart.point.clone());
                }
            }
        }

        collisions
    }

    fn print(&self) {
        let mut karts = HashSet::new();
        for kart in self.karts.as_slice() {
            karts.insert(kart);
        }

        for y in 0..=self.bound.y {
            for x in 0..=self.bound.x {
                match karts.get(&Kart::from_position(x, y)) {
                    Some(kart) => {
                        let kart = match kart.direction {
                            Direction::Left => '<',
                            Direction::Right => '>',
                            Direction::Top => '^',
                            Direction::Bottom => 'v',
                            _ => '😱',
                        };

                        print!("{}", kart);
                    },
                    None => {
                        match self.points.get(&Point { x, y, ..Default::default() }) {
                            Some(point) => {
                                let point = match point.intersection {
                                    true => '+',
                                    false => match point.capability {
                                        Capability::Horizontal => '-',
                                        Capability::Vertical => '|',
                                        Capability::All => '*',
                                        _ => '?'
                                    }
                                };

                                print!("{}", point);
                            },
                            None => print!(" "),
                        }
                    }
                }
            }
            println!("");
        }
    }
}

fn main() -> Result<()> {
    let content = fs::read_to_string(FILE_NAME)?;

    let mut grid = Grid::new();

    for (y, line) in content.lines().enumerate() {
        for (x, current_char) in line.chars().enumerate() {
            // point
            if let Some(point) = Point::from(x, y, current_char) {
                grid.add_point(point);
            }

            // kart
            if let Some(kart) = Kart::from(x, y, current_char) {
                grid.add_kart(kart);
            }
        }
    }

    if DEBUG { grid.print(); }

    let mut end = false;
    let mut part1_done = false;
    while !end {
        let collisions = grid.step();
        if collisions.is_empty() { continue; }

        for point in collisions {
            // part1
            if !part1_done {
                println!("--part 1--");
                println!("{},{}", point.x, point.y);

                part1_done = true;
            }

            // part2
            // FIXME: bugguy, the response is wrong
            // if you want to have a clean implementation, you can go there: https://github.com/BurntSushi/advent-of-code/blob/master/aoc13/src/main.rs
            if grid.remove_karts_from_position(&point) <= 1 {
                println!("--part 2--");
                match grid.karts.get(0) {
                    Some(kart) => println!("{},{}", kart.point.x, kart.point.y),
                    None => println!("No more kart!"),
                }
                end = true;
            }
        }
    }

    Ok(())
}
