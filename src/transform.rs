use crate::buffer::Buffer;
use crate::color::Color;

impl Buffer {

    // blit src on self
    // (0, 0) of src goes to (x, y) of self
    pub fn blit(&mut self, src: &Buffer, x: i32, y: i32) -> &mut Self {

        for curr_x in 0..src.width as i32 {

            if x + curr_x < 0 || x + curr_x >= self.width as i32 {
                continue;
            }

            for curr_y in 0..src.height as i32 {

                if y + curr_y < 0 || y + curr_y >= self.height as i32 {
                    continue;
                }

                self.blit_pixel(
                    (curr_x + x) as usize,
                    (curr_y + y) as usize,
                    src.get_pixel(curr_x as usize, curr_y as usize)
                );
            }

        }

        self
    }

    pub fn mask(&mut self, mask: &Buffer) -> &mut Self {

        if self.width != mask.width || self.height != mask.height {
            return self;
        }

        for x in 0..self.width {

            for y in 0..self.height {

                if mask.get_pixel(x, y).a < 255 {
                    self.set_pixel(x, y, Color::rgba(0, 0, 0, 0));
                }

            }

        }

        self
    }

    pub fn crop(&self, x: i32, y: i32, w: i32, h: i32) -> Buffer {

        if w <= 0 || h <= 0 {
            return Buffer::new(0, 0);
        }

        let mut result = Buffer::new(w as usize, h as usize);

        for curr_x in x..x + w {

            if curr_x < 0 || curr_x >= self.width as i32 {
                continue;
            }

            for curr_y in y..y + h {
                
                if curr_y < 0 || curr_y >= self.height as i32 {
                    continue;
                }

                result.set_pixel((curr_x - x) as usize, (curr_y - y) as usize, self.get_pixel(curr_x as usize, curr_y as usize));
            }

        }

        result
    }

}