#[derive(Debug, Copy, Clone)]
pub struct Loc {
    pub x: usize,
    pub y: usize
}

pub struct Map {
    pub map: Vec<Vec<char>>,
    pub start_loc: Loc
}

pub enum Dir {
    Up,
    Down,
    Left,
    Right
}

impl Loc {
    pub fn new(x: usize, y: usize) -> Self {
        Loc {
            x: x,
            y: y
        }
    }

    pub fn move_dir(&mut self, dir: Dir) {
        match dir {
            Dir::Up => {self.y -= 1;},
            Dir::Down => {self.y += 1;},
            Dir::Left => {self.x -= 1;},
            Dir::Right => {self.x += 1;}
        }
    }
}

#[derive(Clone, Debug)]
pub struct Traveller {
    pub curr_loc: Loc,
    pub curr_path: Vec<Loc>
}

impl Traveller {
    pub fn new(start_loc: Loc) -> Self {
        Traveller { curr_loc: start_loc, curr_path: Vec::new() } 
    }

    pub fn get_from_map(&self, loc:Loc , map: &Map) -> char {
        map.map[loc.y][loc.x]
    }

    pub fn get_moves(&self, map: &Map) -> Vec<Dir> {
        let mut dirs = Vec::new();
        // can we move up
        if self.curr_loc.y != 0 {
            if self.try_move(Dir::Up, map) {
                dirs.push(Dir::Up);
            }
        }
        if self.curr_loc.y != (map.map.len()-1) {
            if self.try_move(Dir::Down, map) {
                dirs.push(Dir::Down);
            }
        }
        if self.curr_loc.x != 0 {
            if self.try_move(Dir::Left, map) {
                dirs.push(Dir::Left);
            }
        }
        if self.curr_loc.x != (map.map[0].len() - 1) {
            if self.try_move(Dir::Right, map) {
                dirs.push(Dir::Right);
            }
        }
        return dirs;
    }

    fn try_move(&self, dir: Dir, map: &Map) -> bool {
        let our_elev = self.get_from_map(self.curr_loc, map);
        let mut next_loc = self.curr_loc.clone();
        next_loc.move_dir(dir);
        let next_elev = self.get_from_map(next_loc, map);
        return Self::is_one_up(our_elev, next_elev);
    } 

    fn is_one_up(curr_elev: char, next_elev: char) -> bool {
        let test_curr_elev = match curr_elev {
            'S' => 'a',
            'E' => 'z',
            c => c, 
        };

        let test_next_elev = match next_elev {
            'S' => 'a',
            'E' => 'z',
            c => c
        };
        return test_curr_elev as u32 + 1 == test_next_elev as u32
    }
}