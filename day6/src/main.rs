mod msg_grabber;
use crate::msg_grabber::MsgGrabber;
use std::fs;

fn main() {
    let message_contents = fs::read_to_string("/home/katielim/advent_of_code_2022/day6/input")
        .expect("Couldn't read file");

    part1(&message_contents[..]);
    part2(&message_contents[..]);
}

fn part1(msg: &str) {
    let mut grabber = MsgGrabber::new(4);
    // we need to add one, because we return the index, but we want the number of characters
    let start_code_end = grabber.find_start_code(msg) + 1;
    let start_code_token = &msg[grabber.start_cursor..(start_code_end)];
    println!("Start code token is {start_code_token:} ending at {start_code_end:}");
}

fn part2(msg: &str) {
    let mut grabber = MsgGrabber::new(14);
    let start_code_end = grabber.find_start_code(msg) + 1;
    let start_code_token = &msg[grabber.start_cursor..(start_code_end)];
    println!("Start code token is {start_code_token:} ending at {start_code_end:}");
}
