extern crate num;
#[macro_use]
extern crate num_derive;
use std::io::{BufRead,BufReader};
use std::fs::File;
use substring::Substring;

mod elf_move;

fn get_score(our_move: &elf_move::Mover, their_move: &elf_move::Mover) -> u32 {
    let our_result = our_move.get_result(their_move); 
    let our_score = (our_result as u32) + (our_move.elf_move as u32);
//    println!("our_result: {our_result:?}, our_score: {our_score:?}");
    our_score
}

fn main() {
    // NOTE: we could use the ? operator on this, but then we need to change the
    // function signature of main to allow returning an io::Result()
    // Instead, let's just unwrap manually and print 
    let f = match File::open("/home/katielim/advent_of_code_2022/day2/input") {
        Err(e) => panic!("Error opening file: {e:?}"),
        Ok(f) => f,
    };
    let reader = BufReader::new(f);
 
    let mut score_sum: u32 = 0;
    for line in reader.lines() {
        let unwrapped = line.unwrap();
        let their_move = elf_move::Mover::new(unwrapped.substring(0,1));
        let result = elf_move::RoundResult::new(unwrapped.substring(2,3));
        let our_next_move = their_move.get_move_fr_result(result);
        let our_move = elf_move::Mover {elf_move: our_next_move};
//        println!("result: {result:?}, our_move: {our_move:?}, their_move: {their_move:?}");
        score_sum += get_score(&our_move, &their_move);
    }
    println!("{score_sum}");
}
