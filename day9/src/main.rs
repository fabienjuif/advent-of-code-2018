// I use i64 there because the scores are too damn high with part2
// And with i32 it causes an overflow

use fnv::FnvHashMap;
use std::collections::HashMap;
use std::fs;
use regex::Regex;

const FILE_NAME: &str = "./input.txt";

#[derive(Debug)]
struct Player {
    id: i64,
    score: i64,
}

#[derive(Clone, Debug, Default)]
struct Marble {
    id: i64,
    next: Option<i64>,
    previous: Option<i64>,
}

#[derive(Default, Debug)]
struct Playground {
    last_marble: i64,
    marbles: FnvHashMap<i64, Marble>,
    current_marble: i64,
}

impl Playground {
    fn create (last_marble: i64) -> Playground {
        Playground {
            last_marble,
            current_marble: -1,
            marbles: FnvHashMap::default(),
        }
    }

    fn rearange(&mut self, mut marble: Marble, next: i64, previous: i64) -> Marble {
        marble.next = Some(next);
        marble.previous = Some(previous);

        let next_marble = self.marbles.get_mut(&next).unwrap();
        next_marble.previous = Some(marble.id);

        let next_marble = self.marbles.get_mut(&previous).unwrap();
        next_marble.next = Some(marble.id);

        marble
    }

    fn add_marble(&mut self, mut marble: Marble) {
        // first marble case
        if self.current_marble == -1 {
            let id = marble.id;

            marble.next = Some(id);
            marble.previous = Some(id);

            self.marbles.insert(id, marble);
            self.current_marble = id;

            return;
        }

        // find the current marble
        let current_marble = self.marbles.get(&self.current_marble).unwrap();

        // take next
        let next_marble = self.marbles.get_mut(&current_marble.next.unwrap()).unwrap();

        // rearange references
        let next = next_marble.next.unwrap();
        let previous = next_marble.id;
        let marble = self.rearange(marble, next, previous);

        // the current_marble becomes the new inserted one
        self.current_marble = marble.id;

        // insert new marble
        self.marbles.insert(marble.id, marble);
    }

    fn get_marble(&mut self, marble: Marble) -> i64 {
        // get the 7 marble counter-clockwise from the current
        let mut to_remove_id = self.current_marble;
        let mut to_remove_previous_id = -1;
        let mut to_remove_next_id = -1;
        for _ in 0..7 {
            let to_remove = self.marbles.get(&to_remove_id).unwrap();
            to_remove_id = to_remove.previous.unwrap();

            let to_remove_previous = self.marbles.get(&to_remove_id).unwrap();
            to_remove_previous_id = to_remove_previous.previous.unwrap();
            to_remove_next_id = to_remove_previous.next.unwrap();
        }

        // the next marble from the one that is found becomes the current marble
        self.current_marble = to_remove_next_id;
        let current_marble = self.marbles.get_mut(&to_remove_next_id).unwrap().clone();

        // rearange references
        let next_id = current_marble.next.unwrap();
        let current_marble = self.rearange(current_marble, next_id, to_remove_previous_id);
        self.marbles.insert(current_marble.id, current_marble);

        // remove the marble found
        self.marbles.remove(&to_remove_id);

        marble.id + to_remove_id
    }
}

impl Marble {
    fn create(id: i64) -> Marble {
        Marble {
            id,
            ..Marble::default()
        }
    }
}

fn play(player_count: i64, last_marble: i64) -> (i64, i64) {
    let mut players = vec![];
    let mut playground = Playground::create(last_marble);

    for index in 0..player_count {
        players.push(Player {
            id: index as i64,
            score: 0,
        });
    }

    let mut next_marble = -1;
    loop {
        if next_marble > last_marble { break; }

        for player in players.iter_mut() {
            next_marble += 1;
            if next_marble > last_marble { break; }

            let marble = Marble::create(next_marble);

            if next_marble != 0 && next_marble % 23 == 0 {
                let score = playground.get_marble(marble);
                player.score += score;
            } else {
                playground.add_marble(marble);
            }
        }
    }

    players.sort_by_key(|player| player.score);

    let winner = players.pop().unwrap();

    (winner.id, winner.score)
}

fn main() {
    let content = fs::read_to_string(FILE_NAME).unwrap();
    let re = Regex::new(r"(.+) players; last marble is worth (.+) points.*").unwrap();
    let captures = re.captures(content.as_str()).unwrap();
    let player_count = captures[1].parse::<i64>().unwrap();
    let last_marble = captures[2].parse::<i64>().unwrap();

    println!("--part 1--");
    let (id, score) = play(player_count, last_marble);
    println!("Player {} as the highest score: {}", id, score);

    println!("--part 2--");
    let (id, score) = play(player_count, last_marble * 100);
    println!("Player {} as the highest score: {}", id, score);
}
