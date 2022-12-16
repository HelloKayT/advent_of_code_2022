
#[derive(Debug, Copy, Clone, FromPrimitive)]
pub enum ElfMove {
    Rock = 1,
    Paper = 2,
    Scissors = 3
}

impl ElfMove {
    fn incr(&self) -> ElfMove {
        let mut next_val = *(self) as u32 + 1;
        if next_val == 4 {
            next_val = 1;
        }

        num::FromPrimitive::from_u32(next_val).unwrap()
    }

    fn decr(&self) -> ElfMove {
        let mut next_val: u32 = *(self) as u32 - 1;
        if next_val == 0 {
            next_val = 3;
        }
        num::FromPrimitive::from_u32(next_val).unwrap()
    }
}

// use `as` to get the values as u32 (or whatever you want)
#[derive(Debug, Copy, Clone)]
pub enum RoundResult {
    Win = 6,
    Draw = 3,
    Loss = 0
}

impl RoundResult {
    pub fn new(result_str: &str) -> RoundResult {
        match result_str {
            "X" => RoundResult::Loss,
            "Y" => RoundResult::Draw,
            "Z" => RoundResult::Win,
            &_ => panic!("Help")
        }
    }
}

#[derive(Debug)]
pub struct Mover {
    pub elf_move: ElfMove
}

impl Mover {
    pub fn new(move_str: &str) -> Self {
        match move_str {
            "A" => Self {elf_move: ElfMove::Rock},
            "B" => Self {elf_move: ElfMove::Paper},
            "C" => Self {elf_move: ElfMove::Scissors},
            "X" => Self {elf_move: ElfMove::Rock},
            "Y" => Self {elf_move: ElfMove::Paper},
            "Z" => Self {elf_move: ElfMove::Scissors},
             &_ => panic!("Help")
        }
    }

    pub fn get_result(&self, other: &Mover) -> RoundResult {
        match self.elf_move {
            ElfMove::Rock => match other.elf_move {
                    ElfMove::Rock => RoundResult::Draw,
                    ElfMove::Scissors => RoundResult::Win,
                    ElfMove::Paper => RoundResult::Loss
                }
            ElfMove::Paper => match other.elf_move {
                ElfMove::Rock => RoundResult::Win,
                ElfMove::Scissors => RoundResult::Loss,
                ElfMove::Paper => RoundResult::Draw,
            }
            ElfMove::Scissors => match other.elf_move {
                ElfMove::Rock => RoundResult::Loss,
                ElfMove::Scissors => RoundResult::Draw,
                ElfMove::Paper => RoundResult::Win,
            }
        }
    }

    pub fn get_move_fr_result(&self, result: RoundResult) -> ElfMove {
        match result {
            RoundResult::Win => self.elf_move.incr(),
            RoundResult::Draw => self.elf_move,
            RoundResult::Loss => self.elf_move.decr()
        }
    }
}
