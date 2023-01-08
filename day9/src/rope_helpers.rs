#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Dirs{
    Up,
    Down,
    Right,
    Left,
    Same
}

impl Dirs {
    pub fn from_string(dir: &str) -> Dirs {
        match dir {
            "R" => Dirs::Right,
            "L" => Dirs::Left,
            "U" => Dirs::Up,
            "D" => Dirs::Down,
            &_ => panic!("unparsable direction")
        }
    }
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Point{
            x: x,
            y: y
        }
    }

    pub fn is_adjacent(&self, other: &Point) -> bool {
        let x_diff = self.x.abs_diff(other.x);
        let y_diff = self.y.abs_diff(other.y);

        //println!("x_diff: {x_diff:}, y_diff: {y_diff:}");
        (self.x.abs_diff(other.x) <= 1) && (self.y.abs_diff(other.y) <= 1)
    }

    pub fn get_x_dir(&self, other: &Point) -> Dirs {
        if self.x == other.x {
            return Dirs::Same;
        }
        else if self.x < other.x {
            return Dirs::Right;
        }
        else {
            return Dirs::Left;
        }
    }

    pub fn get_y_dir(&self, other: &Point) -> Dirs {
        if self.y == other.y {
            return Dirs::Same;
        }
        else if self.y < other.y {
            return Dirs::Up;
        }
        else {
            return Dirs::Down;
        }
    }

    pub fn step_point(&mut self, dir: Dirs) {
        match dir {
            Dirs::Up => {
                self.y += 1;
            },
            Dirs::Down => {
                self.y -= 1;
            }, 
            Dirs::Right => {
                self.x += 1;
            },
            Dirs::Left => {
                self.x -= 1;
            },
            Dirs::Same => {//do nothing
            }
        }
    } 
}

pub struct Rope {
    pub knots: Vec<Point>
}

impl Rope {
    pub fn new(start_loc: Point, num_knots: usize) -> Self {
        Rope {
            knots: vec![start_loc; num_knots]
        }
    }

    pub fn step_head(&mut self, dir: Dirs) {
        self.knots[0].step_point(dir);
    }

    pub fn step_trailing_knot(&mut self, knot_num: usize) {
        assert!(knot_num >= 1);
        if !self.knots[knot_num].is_adjacent(&self.knots[knot_num-1]) {
            let y_dir = self.knots[knot_num].get_y_dir(&self.knots[knot_num-1]);
            let x_dir = self.knots[knot_num].get_x_dir(&self.knots[knot_num-1]);

            self.knots[knot_num].step_point(y_dir);
            self.knots[knot_num].step_point(x_dir);
        }
    }
}