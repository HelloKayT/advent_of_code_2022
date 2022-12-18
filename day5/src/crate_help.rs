use std::cell::RefCell;
use std::rc::Rc;
#[derive(Debug, Clone)]
pub struct MoveLine {
    pub num_crates: usize,
    pub src: usize,
    pub dst: usize
}

pub fn move_line_lexer(input: &str) -> MoveLine {
    // split the string
    let mut iter = input.split(" ");
    let mut num_crates = 0;
    let mut src = 0;
    let mut dst = 0;
    loop {
        match iter.next() {
            Some(token) => {
                match token {
                    "move" => { 
                        match iter.next() {
                            Some(number) => num_crates = number.parse().unwrap(),
                            None => panic!("lexing error")
                        };
                    }
                    "from" => {
                        match iter.next() {
                            Some(number) => src = number.parse().unwrap(),
                            None => panic!("lexing error")
                        };
                    }
                    "to" => {
                        match iter.next() {
                            Some(number) => dst = number.parse().unwrap(),
                            None => panic!("lexing error")
                        }
                    }
                    &_ => {
                        panic!("unexpected token")
                    }
                }
            },
            None => {
                break;
            }
        };
    }
    MoveLine {num_crates: num_crates, src: src, dst: dst}
}

#[derive(Debug)]
pub struct CrateStack {
    pub crates: Vec<RefCell<Vec<Crate>>>,
    pub stack_width: usize
}

pub enum CrateLexResult {
    CrateNumRow(usize),
    CrateError,
    CrateOk,
}

#[derive(Debug,Clone, Copy)]
pub struct Crate {
    pub contents: char
}

impl CrateStack {
    pub fn new(num_stacks: usize, stack_width: usize) -> Self {
        let mut new_struct = CrateStack{
            crates: Vec::new(),
            stack_width: stack_width 
        };

        for _i in 0..num_stacks {
            new_struct.crates.push(RefCell::new(Vec::<Crate>::new()));
        }
        return new_struct;
    }

    fn get_crate_index(&self, cursor: usize) -> usize {
        if (cursor % (self.stack_width + 1) != 0) {
            panic!("Cursor misaligned")
        }
        cursor/(self.stack_width + 1)
    }

    pub fn crate_line_lexer(&mut self, line: &str) -> Result<CrateLexResult, &str>{
        let stack_chars: Vec<char> = line.chars().collect();
        let mut cursor = 0;
        while cursor < stack_chars.len() {
            let stack_char = stack_chars[cursor];
            match stack_char {
                '[' => {
                    // which index are we at
                    let crate_index = self.get_crate_index(cursor);
                    let new_crate = Crate {contents: stack_chars[cursor + 1]};
                    (self.crates[crate_index]).borrow_mut().insert(0, new_crate);
                    // advance the cursor by 2 to consume the token: char and closing
                    // bracket
                    cursor += 2
                }
                _ => {
                    if stack_char.is_digit(10) {
                        // go find the last digit in the line
                        let digits: Vec<&str> = line.split_whitespace().collect();
                        let last_digit = digits.last().unwrap().parse().unwrap();
                        return Ok(CrateLexResult::CrateNumRow(last_digit));
                    } 
                    else if !stack_char.is_whitespace() {
                        return Err("Bad character in line: {stack_char}");
                    }
                }
            }
            cursor += 1;
        }
        Ok(CrateLexResult::CrateOk)
    }
}