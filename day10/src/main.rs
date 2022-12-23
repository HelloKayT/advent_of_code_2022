mod cpu_helper;
mod crt_helper;

use std::io::{
    BufReader
};
use std::fs::File;


use crate::{
    cpu_helper::{
        CPU,
        TickResult
    },
    crt_helper::CRT
};

fn main() {
    let f = match File::open("/home/katielim/advent_of_code_2022/day10/input") {
        Err(e) => panic!("Error opening file: {e:?}"),
        Ok(f) => f,
    };
    let reader = BufReader::new(f);

    part2(reader);
}

fn part1(reader: BufReader<File>) {
    let mut cpu = CPU::new(reader);
    let mut cycle_count: u64 = 1;
    let mut next_print_cycle = 20;
    let mut cycle_value: Vec<i64> = Vec::new();
    let mut signal_strength: Vec<i64> = Vec::new();
    loop {
        println!("====Cycle {cycle_count:}====");
        if cycle_count == next_print_cycle {
            let reg_val = cpu.registers.get("X").unwrap().clone();
            cycle_value.push(reg_val);
            signal_strength.push(reg_val * (cycle_count as i64));
            next_print_cycle += 40;
        }
        cpu.dump_regs();
        match cpu.tick() {
            TickResult::Running => {
                cycle_count += 1
            },
            TickResult::Done => {break}
        };
    }
    println!("{cycle_value:?}");
    let signal_product = signal_strength.into_iter().reduce(|a, b| a + b);
    println!("Total signal product: {signal_product:?}");
}

fn part2(reader: BufReader<File>) {
    let mut cpu = CPU::new(reader);
    let mut crt = CRT::new(40, 6, 3);
    let mut cycle_count = 1;
    loop {
        println!("====Cycle {cycle_count:}====");
        let sprite_position = cpu.registers.get("X").unwrap().clone();
        cpu.dump_regs();
        crt.tick(sprite_position);
        crt.print_screen();
        match cpu.tick() {
            TickResult::Running => {
                cycle_count += 1
            },
            TickResult::Done => {break}
        };
    }
}

