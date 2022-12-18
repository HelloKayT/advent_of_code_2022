mod crate_help;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;
use std::io::{
    BufRead, 
    BufReader
};
use std::fs::File;
use crate_help::{move_line_lexer, MoveLine};
use crate::crate_help::CrateStack;

fn main() {
    let f = match File::open("/home/katielim/advent_of_code_2022/day5/input") {
        Err(e) => panic!("Error opening file: {e:?}"),
        Ok(f) => f,
    };
    let mut reader = BufReader::new(f);
    run_loop(&mut reader)
}

fn run_loop(reader: &mut BufReader<File>) {
    let mut iter = reader.lines();
    let mut parsing_commands = false;
    // figure out how many crate stacks there are
    let mut unwrapped: String = iter.next().unwrap().unwrap();
    let stack_width = 3;
    let num_stacks = (unwrapped.len() + 1)/(stack_width + 1);
    let mut crate_stack = CrateStack::new(num_stacks, 3);

    loop {
        if unwrapped.is_empty() {
            println!("Moving to parse commands");
            parsing_commands = true;
        }
        else {
            if (parsing_commands) {
                let move_line_cmd = move_line_lexer(&unwrapped);
                part2_no_extra_copy(&move_line_cmd, &mut crate_stack);
            }
            else {
                match crate_stack.crate_line_lexer(&unwrapped) {
                    Ok(_) =>  {},
                    Err(e) => {panic!("{e:}")}
                }
            }
        }
        match iter.next() {
            Some(line) => unwrapped = line.unwrap(),
            None => break
        } 
    }
    // loop over the crates to print out
    let mut final_string: String = String::from("");
    for stack in crate_stack.crates {
        match (*stack).borrow().last() {
            Some(last_crate) => {
                final_string.push(last_crate.contents);
            }
            None => {

            }
        }
    }
    println!("Top crates: {final_string}");
}

fn part1_rearrange(line_cmd: &MoveLine, crate_stack: &mut CrateStack) {
    // act on the command
    for _i in 0..line_cmd.num_crates {
        match (*(crate_stack.crates[line_cmd.src - 1])).borrow_mut().pop() {
            Some(Crate) => (*(crate_stack.crates[line_cmd.dst - 1])).borrow_mut().push(Crate),
            None => panic!("Popped from empty stack")
        }
    }
}

fn part2_no_extra_copy(line_cmd: &MoveLine, crate_stack: &mut CrateStack) {
    let mut move_index: usize = 0;
    {
        let src_stack_len = (*(crate_stack.crates[line_cmd.src-1])).borrow().len();
        move_index = src_stack_len - line_cmd.num_crates;
        let src_slice = &(*(crate_stack.crates[line_cmd.src-1])).borrow()[move_index..src_stack_len];
        (*(crate_stack.crates[line_cmd.dst-1])).borrow_mut().extend_from_slice(src_slice);
    }
    (*(crate_stack.crates[line_cmd.src-1])).borrow_mut().truncate(move_index);
}


