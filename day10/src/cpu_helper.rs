use std::{collections::HashMap, io::{BufReader, Lines, BufRead}, fs::File};

#[derive(Debug)]
pub enum Instruction {
    Noop,
    Addx(String, i64)
}

impl Instruction {
    fn get_delay(&self) -> u64 {
        match self {
            Instruction::Noop => 1,
            Instruction::Addx(_,_) => 2,
        }
    }

    fn from_string(inst_str: String) -> Self {
        let tokens: Vec<&str> = inst_str.split_whitespace().collect();
        match tokens[0] {
            "noop" => Instruction::Noop,
            "addx" => Instruction::Addx(String::from("X"), tokens[1].parse().unwrap()),
            &_ => panic!("Bad instruction string")
        }
    }
}

pub struct CPU {
    pub registers: HashMap<&'static str, i64>,
    inst_stream: Lines<BufReader<File>>,
    curr_inst: Instruction,
    curr_state: CPUState,
    state_cycle_count: u64
}

pub enum CPUState { 
    Ready,
    InstExecute,
}

pub enum TickResult {
    Running,
    Done
}

impl CPU {
    pub fn new(reader: BufReader<File>) -> Self {
        let mut new_hashmap = HashMap::new();
        new_hashmap.insert("X", 1);

        CPU {
            registers: new_hashmap,
            inst_stream: reader.lines(),
            curr_inst: Instruction::Noop,
            curr_state: CPUState::Ready,
            state_cycle_count: 0
        }
    }

    pub fn tick(&mut self) -> TickResult {
        match self.curr_state {
            CPUState::Ready => {
                match self.inst_stream.next() {
                    Some(line_res) => {
                        let inst_line = line_res.unwrap();
                        self.curr_inst = Instruction::from_string(inst_line);

                        if self.state_cycle_count == (self.curr_inst.get_delay() - 1) {
                            self.apply_instruction();
                            self.state_cycle_count = 0;
                            // don't change CPU state
                        }
                        else {
                            self.state_cycle_count += 1;
                            self.curr_state = CPUState::InstExecute;
                        }
                        return TickResult::Running;
                    },
                    None => {return TickResult::Done;}
                }
            },
            CPUState::InstExecute => {
                if self.state_cycle_count == (self.curr_inst.get_delay() - 1) {
                    self.apply_instruction();
                    self.state_cycle_count = 0;
                    self.curr_state = CPUState::Ready;
                }
                else {
                    self.state_cycle_count += 1;
                }
                return TickResult::Running;
            }
        }
    }

    fn apply_instruction(&mut self) {
        match &self.curr_inst {
            Instruction::Noop => {},
            Instruction::Addx(reg, operand) => {
                let register_val = self.registers.get_mut(reg.as_str())
                    .expect("couldn't find register");
                *register_val += operand;
            }
        }
    }

    pub fn dump_regs(&self) {
        for (key, val) in &self.registers {
            println!("reg {key:}: {val:}");
        }
    }
}