#[derive(Default, Debug)]
struct Cell {
    x: usize,
    y: usize,
    rack_id: usize,
    power_level: isize,
}

impl Cell {
    fn new(x: usize, y: usize, serial: usize) -> Cell {
        let rack_id = x + 10;

        let mut power_level = rack_id as isize * y as isize;
        power_level += serial as isize;
        power_level *= rack_id as isize;

        if power_level < 100 {
            power_level = 0;
        } else {
            power_level = ((power_level % 1000) - (power_level % 100)) / 100;
        }

        power_level -= 5;

        Cell {
            x,
            y,
            rack_id,
            power_level,
            ..Default::default()
        }
    }
}

struct Grid {
    cells: Vec<Cell>,
}

impl Grid {
    fn new(serial: usize, bound: usize) -> Grid {
        let mut cells = vec![];

        for x in 1..=bound {
            for y in 1..=bound {
                cells.push(Cell::new(x, y, serial));
            }
        }

        Grid { cells }
    }

    fn get_total_power(&self, bound: usize, x: usize, y: usize, size: usize) -> isize {
        let mut total = 0;

        for x in x..x+size {
            for y in y..y+size {
                total += self.cells.get(x * bound + y).unwrap().power_level;
            }
        }

        total
    }
}

fn part1(grid: &Grid, bound: usize) {
    let mut max_power = 0;
    let mut max_x = 0;
    let mut max_y = 0;

    for x in 0..bound - 3 {
        for y in 0..bound - 3 {
            let current_power = grid.get_total_power(bound, x, y, 3);
            if current_power > max_power {
                max_power = current_power;
                max_x = x + 1;
                max_y = y + 1;
            }
        }
    }

    println!("--part 1--");
    println!("[{},{}]", max_x, max_y);
}

fn part2(grid: &Grid, bound: usize) {
    let mut max_power = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_size = 1;

    for size in 1..bound {
        for x in 0..bound - size {
            for y in 0..bound - size {
                let current_power = grid.get_total_power(bound, x, y, size);
                if current_power > max_power {
                    max_power = current_power;
                    max_x = x + 1;
                    max_y = y + 1;
                    max_size = size;
                }
            }
        }
    }

    println!("--part 1--");
    println!("[{},{},{}]", max_x, max_y, max_size);
}

fn main() {
    // debug
    assert_eq!(Cell::new(3, 5, 8).power_level, 4);
    assert_eq!(Cell::new(122, 79, 57).power_level, -5);
    assert_eq!(Cell::new(217, 196, 39).power_level, 0);
    assert_eq!(Cell::new(101, 153, 71).power_level, 4);

    // construction
    let bound = 300;
    let grid = Grid::new(9424, bound);

    // part1
    part1(&grid, bound);

    // part2
    part2(&grid, bound);
}
