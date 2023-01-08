mod grid_helpers;

use std::io::{
    BufRead, 
    BufReader
};
use std::fs::File;

use crate::grid_helpers::{
    TreeGrid, Tree
};

fn main() {
    let f = match File::open("/home/katielim/advent_of_code_2022/day8/input") {
        Err(e) => panic!("Error opening file: {e:?}"),
        Ok(f) => f,
    };
    let mut reader = BufReader::new(f);
    let mut lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        lines.push(line.unwrap());
    }

    let trees = TreeGrid::new(&lines);
    part1(&trees);
    part2(&trees);
}

fn is_seen(tree: &Tree) -> bool {
    return tree.is_seen.borrow().clone();
}

fn part1(trees: &TreeGrid) {
    for y in 0..trees.height {
        for x in 0..trees.width {
            trees.mark_tree_is_seen(x, y);
        }
    }

    //trees.print_grid();

    let trees_seen = trees.count_true(is_seen);
    println!("trees_seen: {trees_seen:}");
}

fn part2(trees: &TreeGrid) {
    let mut max_scenic_score = 0;

    //max_scenic_score = trees.get_tree_scenic_score(3, 2);

    // don't check the edge trees
    for y in 1..(trees.height-1) {
        for x in 1..(trees.width-1) {
            let scenic_score = trees.get_tree_scenic_score(x, y);
            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }
    println!("scenic_score: {max_scenic_score:}");
}