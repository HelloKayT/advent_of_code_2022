use std::{cell::RefCell, ops::Range, borrow::BorrowMut};

#[derive(Debug)]
pub struct Tree {
    pub size: u32,
    pub is_seen: RefCell<bool>,
}

impl Tree {
    pub fn new(size: u32) -> Self {
        Tree {
            size: size,
            is_seen: RefCell::new(false)
        }
    }
}

pub struct TreeGrid {
    array: Vec<Vec<Tree>>,
    pub width: usize,
    pub height: usize
}

impl TreeGrid {
    pub fn new(trees: &Vec<String>) -> Self {
        let mut tree_vec: Vec<Vec<Tree>> = Vec::new();
        for row in trees {
            let mut tree_row: Vec<Tree> = Vec::new();
            for tree in row.chars() {
                let tree_size = tree.to_digit(10).unwrap();
                tree_row.push(Tree::new(tree_size));
            }
            tree_vec.push(tree_row);
        }
        let width = tree_vec[0].len();
        let height = tree_vec.len();
        let grid = TreeGrid { 
            array: tree_vec,
            width: width,
            height: height
        };
        grid.mark_edges();
        return grid;
    }

    fn mark_edges(&self) {
        // mark first and last rows
        for tree in &(self.array[0]) {
            *(tree.is_seen.borrow_mut()) = true;
        }
        for tree in &(self.array[self.height-1]) {
            *(tree.is_seen.borrow_mut()) = true;
        }

        // mark first and last columns
        for index in 0..self.height {
            let tree = self.get_tree(0, index);
            *(tree.is_seen.borrow_mut()) = true;
        }
        for index in 0..self.height {
            let tree = self.get_tree(self.width-1, index);
            *(tree.is_seen.borrow_mut()) = true;
        }
    }

    pub fn get_tree(&self, x: usize, y: usize) -> &Tree {
        &(self.array[y][x])
    }

    pub fn mark_tree_is_seen(&self, x: usize, y: usize) {
        let mut cell_tree_is_seen = self.get_tree(x, y).is_seen.borrow_mut();
        // from top
        let top_max = self.col_max_index(x, 0..y+1);
        if top_max == y{
            *(cell_tree_is_seen) = true;
        }
        // from bottom
        let bot_max = self.col_max_index(x, (y..self.height).rev());
        if bot_max == y {
            *(cell_tree_is_seen) = true;
        }

        // from left
        let left_max = self.row_max_index(y, 0..x+1);
        if left_max == x {
            *(cell_tree_is_seen) = true;
        }
        // from right
        let right_max = self.row_max_index(y, (x..self.width).rev());
        if right_max == x {
            *(cell_tree_is_seen) = true;
        }
    }

    pub fn col_max_index<I: Iterator<Item=usize>>(&self, x: usize, iter: I) -> usize {
        let mut max_num: u32 = 0;
        let mut max_index: usize = 0;

        for i in iter {
            let cell_val = self.get_tree(x, i).size;
            if  cell_val > max_num {
                max_num = cell_val;
                max_index = i;
            }
        }
        return max_index;
    }

    pub fn row_max_index<I: Iterator<Item=usize>>(&self, y: usize, iter: I) -> usize {
        let mut max_num: u32 = 0;
        let mut max_index: usize = 0;

        for i in iter {
            let cell_val = self.get_tree(i, y).size;
            if  cell_val > max_num {
                max_num = cell_val;
                max_index = i;
            }
        }
        return max_index;
    }

    pub fn count_true(&self, f: fn(&Tree) -> bool) -> usize {
        let mut tree_count = 0;
        for tree_row in &(self.array) {
            for tree in tree_row {
                if f(tree) {
                    tree_count += 1;
                }
            }
        }
        return tree_count;
    }

    pub fn print_grid(&self) {
        for row in &(self.array) {
            println!("{row:?}");
        }
    }

    pub fn find_blocked_col<I: Iterator<Item=usize>>(&self, x: usize, y: usize, iter: I) -> usize {
        let tree_size = self.get_tree(x, y).size;
        for index in iter {
            if self.get_tree(x, index).size >= tree_size {
                return index;
            }
        }
        return y;
    }

    pub fn find_blocked_row<I: Iterator<Item=usize>>(&self, x: usize, y: usize, iter: I) -> usize {
        let tree_size = self.get_tree(x, y).size;
        for index in iter {
            if self.get_tree(index, y).size >= tree_size {
                return index;
            }
        }
        return x;
    }

    pub fn get_tree_scenic_score(&self, x: usize, y: usize) -> u32 {
        let mut scenic_score = 1;
        let top_max = self.find_blocked_col(x, y, (0..y).rev());
        // we can see all the way to the edge
        if top_max == y {
            scenic_score *= y;
        }
        else {
            scenic_score *= y - top_max;
        }
        // from bottom
        let bot_max = self.find_blocked_col(x, y, (y+1)..self.height);
        if bot_max == y {
            scenic_score *= self.height - 1 - y;
        }
        else {
            scenic_score *= bot_max - y;
        }

        // from left
        let left_max = self.find_blocked_row(x, y, (0..x).rev());
        if left_max == x {
            scenic_score *= x;
        }
        else {
            scenic_score *= x - left_max;
        }
        // from right
        let right_max = self.find_blocked_row(x, y, (x+1)..self.width);
        if right_max == x {
            scenic_score *= self.width - 1 - x;
        }
        else {
            scenic_score *= right_max - x;
        }
        return scenic_score as u32;
    }
}