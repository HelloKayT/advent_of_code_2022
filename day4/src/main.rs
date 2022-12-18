mod range_help;
use std::io::{
    BufRead, 
    BufReader
};
use std::fs::File;
use crate::range_help::RangeHelper;

fn main() {
    let f = match File::open("/home/katielim/advent_of_code_2022/day4/input") {
        Err(e) => panic!("Error opening file: {e:?}"),
        Ok(f) => f,
    };
    let mut reader = BufReader::new(f);

    run_loop(&mut reader);
}

fn run_loop(reader: &mut BufReader<File>) {
    let mut range_sum: u32 = 0;
    // parse each line into ranges
    for line in reader.lines() {
        let unwrapped = line.unwrap();
        let (range1, range2) = match unwrapped.split_once(",") {
            Some((int1, int2)) => (int1, int2),
            None => panic!("Parsing error")
        };

        let range_help1 = RangeHelper::new(range1);
        let range_help2 = RangeHelper::new(range2);

        if part2(&range_help1, &range_help2) {
            range_sum += 1;
//            println!("{unwrapped}");
        }
    }
    println!("range_sum: {range_sum}");
}

fn part1(range1: &RangeHelper, range2: &RangeHelper) -> bool {
    return range1.contains(range2) || range2.contains(range1)
} 

fn part2(range1: &RangeHelper, range2: &RangeHelper) -> bool {
    return range1.overlaps(range2)
}

