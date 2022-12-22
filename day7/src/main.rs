mod fs_helpers;
mod fs_builder;
use std::io::{
    BufRead, 
    BufReader
};
use std::fs::File;
use std::rc::Rc;

use fs_builder::build_fs;
use fs_helpers::DirNode;
fn main() {
    let f = match File::open("/home/katielim/advent_of_code_2022/day7/input") {
        Err(e) => panic!("Error opening file: {e:?}"),
        Ok(f) => f,
    };
    let mut reader = BufReader::new(f);
    let root_node = build_fs(&mut reader);
    root_node.update_size();
    //println!("{root_node:?}");

//    part1(root_node);
    part2(root_node);
}

fn less_than(size: usize, threshold: usize) -> bool {
    size <= threshold
}

fn greater_than(size: usize, threshold: usize) -> bool {
    size >= threshold
}

fn part1(root_node: Rc<DirNode>) {
    let mut dirs_less_than = root_node.filter_child_dirs(less_than, 100000);
    // we need to check root
    if root_node.get_size() <= 100000 {
        dirs_less_than.push(root_node);
    }
    let mut total_size_sum = 0;
    for dir in dirs_less_than {
        let dir_size = (*dir).get_size();
        println!("size: {dir_size:?}");
        total_size_sum += (*dir).get_size();
    }
    println!("total_size: {total_size_sum:}");
}

fn part2(root_node: Rc<DirNode>) {
    let total_space: usize = 70000000;
    let target_space: usize = 30000000;

    let used_space = root_node.get_size();
    let free_space = total_space - used_space;
    let need_to_free = target_space - free_space;

    let dirs_greater_than = root_node.filter_child_dirs(greater_than, need_to_free);
    let mut size_vec: Vec<usize> = Vec::new();

    for dir in dirs_greater_than {
        size_vec.push(dir.get_size());
    }

    size_vec.sort();
    let smallest_dir = size_vec[0];
    println!("smallest dir: {smallest_dir:}");
}