use std::io::{BufRead,BufReader};
use std::fs::File;

fn main() {
    // NOTE: we could use the ? operator on this, but then we need to change the
    // function signature of main to allow returning an io::Result()
    // Instead, let's just unwrap manually and print 
    let f = match File::open("/home/katielim/advent_of_code_2022/day1/input") {
        Err(e) => panic!("Error opening file: {e:?}"),
        Ok(f) => f,
    };
    let mut reader = BufReader::new(f);
   
    let mut elves_vec: Vec<u32> = Vec::new();

    let mut running_sum: u32 = 0;
    for line in reader.lines() {
        let unwrapped = line.unwrap();
        if unwrapped.is_empty() {
            elves_vec.push(running_sum);
            running_sum = 0;
        }
        else {
            let cal_count = unwrapped.parse::<u32>().unwrap();
            running_sum += cal_count;
        }
    }
    elves_vec.sort();
//    println!("{elves_vec:?}");
    let max_3: &[u32] = &elves_vec[elves_vec.len()-3..elves_vec.len()];
    let max_total: u32 = max_3.iter().sum();
    println!("{max_total:?}");
}
