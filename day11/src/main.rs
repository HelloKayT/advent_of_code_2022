mod monkey;
use std::cell::{RefCell};
use std::io::{BufRead,BufReader};
use std::fs::File;

use crate::monkey::Monkey;

fn main() {
    let f = match File::open("/home/katielim/advent_of_code_2022/day11/input") {
        Err(e) => panic!("Error opening file: {e:?}"),
        Ok(f) => f,
    };
    let reader = BufReader::new(f);

    let mut monkeys : Vec<RefCell<Monkey>> = Vec::new();
    let mut monkey_lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        let unwrapped = line.unwrap();
        if unwrapped.is_empty() {
            let monkey = Monkey::new(monkey_lines);
            monkeys.push(RefCell::new(monkey)); 
            monkey_lines = Vec::new();
        }
        else {
            monkey_lines.push(unwrapped);
        }
    }
    // push the last monkey
    monkeys.push(RefCell::new(Monkey::new(monkey_lines)));
    println!("monkeys: {monkeys:?}");
    monkey_business(&monkeys, 10000);
}

fn monkey_business(monkeys: &Vec<RefCell<Monkey>>, num_rounds: u64) {
    let mut mod_by = 1;
    for monkey in monkeys {
        mod_by *= monkey.borrow().test_divisor;
    }

    for i in 0..num_rounds {
        println!("Round {i:}");
        monkey_round(monkeys, mod_by);
    }

    let mut num_inspections: Vec<(u64, usize)> = Vec::new();
    for i in 0..monkeys.len() {
        num_inspections.push((monkeys[i].borrow().items_inspected, i));
    }
    // sort vector
    num_inspections.sort_by_key(|k| k.0);

    // get the two largest
    let (monkey_2_count, monkey_2) = num_inspections[monkeys.len()-2];
    println!("Monkey {:} with {:} items", monkey_2, monkey_2_count);
    let (monkey_1_count, monkey_1) = num_inspections[monkeys.len()-1];
    println!("Monkey {:} with {:} items", monkey_1, monkey_1_count);
    println!("Total monkey business: {:}", monkey_1_count * monkey_2_count);
}

fn monkey_round(monkeys: &Vec<RefCell<Monkey>>, mod_by: u64) {
    for monkey in monkeys {
        let mut monkey_borrow = monkey.borrow_mut();
        while let Some(item) = monkey_borrow.items.pop_front() {
            let (dest_monkey, new_item) = monkey_borrow.process_item(item, mod_by);
            //println!("item {new_item:} to monkey {dest_monkey:}");
            (monkeys[dest_monkey as usize]).borrow_mut().items.push_back(new_item);
        }
    }
}


