mod travel_helpers;
use std::io::{BufRead,BufReader};
use std::fs::File;

use travel_helpers::{Traveller, Loc, Map};

fn main() {
    let f = match File::open("/home/katielim/advent_of_code_2022/day11/input") {
        Err(e) => panic!("Error opening file: {e:?}"),
        Ok(f) => f,
    };
    let reader = BufReader::new(f);

    let map = get_map(reader);
    
}

fn traverse(map: &Map) {
    let mut min_traveller: Traveller;
    let mut min_distance = usize::MAX;
    let mut active_travellers: Vec<Traveller> = Vec::new();
    active_travellers.push(Traveller::new(map.start_loc));

    while let Some(traveler) = active_travellers.pop() {
        // check if we've reached the destination
        if traveler.get_from_map(traveler.curr_loc, map) == 'E' {
            // check if we're the min distance
            if traveler.curr_path.len() < min_distance {
                min_traveller = traveler;
            }
        }
        else {
            // should we traverse this traveller? If our current length is
            // greater than or equal to the minimum length, just squash this
            // branch of travel
            if (traveler.curr_path.len() < min_distance) {
                // spawn more travellers
                let potential_dirs = traveler.get_moves(map);
                for dir in potential_dirs {
                    let mut new_traveller = traveler.clone();
                    new_traveller.curr_loc.move_dir(dir);
                    active_travellers.push(new_traveller)
                }
            }
        }

    }
    println!("min traveller: {min_traveller:?}");
}

fn get_map(reader: BufReader<File>) -> Map {
    let mut map = Vec::new();
    let mut start_loc = Loc::new(0, 0);

    for line in reader.lines() {
        let unwrapped = line.unwrap();
        match unwrapped.find("S") {
            Some(index) => {start_loc = Loc::new(index, map.len());}
            None => {}
        };
        map.push(unwrapped.chars().collect());
    }

    return Map{map: map, start_loc: start_loc};
}
