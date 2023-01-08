mod rope_helpers;
use core::num;
use std::hash::Hash;
use std::io::{
    BufRead, 
    BufReader
};
use std::fs::File;
use std::collections::HashMap;

use crate::rope_helpers::{
    Dirs,
    Point,
    Rope
};

fn main() {
    let f = match File::open("/home/katielim/advent_of_code_2022/day9/input") {
        Err(e) => panic!("Error opening file: {e:?}"),
        Ok(f) => f,
    };
    let mut reader = BufReader::new(f);
    //part1(&mut reader);
    part2(&mut reader);
}

fn part1(reader: &mut BufReader<File>) {
    rope_runner(reader, 2);
}

fn part2(reader: &mut BufReader<File>) {
    rope_runner(reader, 10);
}


fn rope_runner(reader: &mut BufReader<File>, num_knots: usize) {
    let mut spaces: HashMap<Point, u32> = HashMap::new();
    let mut rope = Rope::new(Point{x: 0, y:0}, num_knots);
    spaces.insert(Point::new(0, 0), 1);

    for line in reader.lines() {
        let move_line = line.unwrap();
        let moves: Vec<&str> = move_line.split_whitespace().collect();
        let dir = Dirs::from_string(moves[0]);
        let num_steps = moves[1].parse().unwrap();

        println!("===={dir:?}====");
        for i in 0..num_steps {
            println!("===={i:}====");
            rope.step_head(dir);
            println!("head: {:?}", rope.knots[0]);
            for i in 1..num_knots {
                rope.step_trailing_knot(i);
                if i == (num_knots-1) {
                    match spaces.get_mut(&rope.knots[i]) {
                        Some(count) => {
                            (*count) += 1;
                        }
                        None => {
                            spaces.insert(rope.knots[i], 1);
                        }
                    };
                }
                println!("knot {i:}: {:?}", rope.knots[i]);
            }
        }
    }
    let num_uniq = spaces.len();
    println!("num unique spaces: {num_uniq:}");
    //check_input_small(&spaces)
}

fn check_input_small (table: &HashMap<Point, u32>) {
    let solution_points = vec![
        Point::new(0, 0),
        Point::new(1, 0),
        Point::new(2, 0),
        Point::new(3, 0),
        Point::new(4, 1),
        Point::new(1, 2),
        Point::new(2, 2),
        Point::new(3, 2),
        Point::new(4, 2),
        Point::new(3, 3),
        Point::new(4, 3),
        Point::new(2, 4),
        Point::new(3, 4)
    ];

    for point in solution_points {
        if !table.contains_key(&point) {
            println!("Table doesn't contain point: {point:?}");
        }
    }
    println!("{table:?}");
}