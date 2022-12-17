mod item_priority;

use core::num;
use std::io::{
    BufRead, 
    BufReader
};
use std::fs::File;

use crate::item_priority::Rucksack;

fn main() {
    // NOTE: we could use the ? operator on this, but then we need to change the
    // function signature of main to allow returning an io::Result()
    // Instead, let's just unwrap manually and print 
    let f = match File::open("/home/katielim/advent_of_code_2022/day3/input") {
        Err(e) => panic!("Error opening file: {e:?}"),
        Ok(f) => f,
    };
    let mut reader = BufReader::new(f);
    part2(&mut reader);
}

fn part1(reader: &mut BufReader<File>) {
    let num_compartments: u32 = 2;

    let mut total_prio = 0;
    for line in reader.lines() {
        let unwrapped = line.unwrap();
        let contents_len = unwrapped.len()/num_compartments as usize;
        let mut sack = Rucksack::new();

        for i in 0..num_compartments {
            let start = (i as usize *contents_len) as usize;
            let end = ((i as usize + 1)*contents_len) as usize;

            let items = &unwrapped[start..end];
            sack.add_compartment(items);
        }

        total_prio += sack.calc_contents_prio();
    }
    println!("total_prio: {total_prio}");
}

fn part2(reader: &mut BufReader<File>) {
    let num_compartments = 3;
    let mut total_prio = 0;
    let mut curr_index = 0;
    let mut sack: Rucksack = Rucksack::new();

    for line in reader.lines() {
        if curr_index == 0 {
            sack = Rucksack::new();
        }
        let unwrapped = line.unwrap();
        sack.add_compartment(&unwrapped);

        if curr_index == (num_compartments - 1) {
            total_prio += sack.calc_contents_prio();
            curr_index = 0;
        }
        else {
            curr_index += 1;
        }
    } 
    println!("total_prio: {total_prio}");
}
