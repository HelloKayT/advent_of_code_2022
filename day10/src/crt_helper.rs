pub struct CRT {
    width: usize,
    height: usize,
    pixel_vec: Vec<bool>,
    curr_pixel: i64,
    sprite_width: i64,
}

impl CRT {
    pub fn new(width: usize, height: usize, sprite_width: i64) -> Self {
        CRT {
            width: width,
            height: height,
            pixel_vec: vec![false; width*height],
            curr_pixel: 0,
            sprite_width: sprite_width
        }
    }

    pub fn tick(&mut self, sprite_loc: i64) {
        // gotta subtract 1 for zero based indexing
        let sprite_index = (sprite_loc - 1);
        // gotta subtract 1 for inclusive counting
        let sprite_end = (sprite_index + self.sprite_width - 1);
        let x_pixel = self.curr_pixel % self.width as i64;
        if (x_pixel >= sprite_index) && (x_pixel <= sprite_end) {
            self.pixel_vec[self.curr_pixel as usize] = true;
        }

        if self.curr_pixel == (self.pixel_vec.len() - 1).try_into().unwrap() {
            self.curr_pixel = 0;
        }
        else {
            self.curr_pixel += 1;
        }
    }

    pub fn print_screen(&self) {
        for row in 0..self.height {
            for pixel in 0..self.width {
                let pixel_num = (row * self.width) + pixel;
                if self.pixel_vec[pixel_num] {
                    print!("#")
                }
                else {
                    print!(".")
                }
            }
            println!("");
        }
    }
}
