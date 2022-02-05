use crate::buffer::Buffer;
use crate::color::Color;

impl Buffer {

    pub fn blit(&mut self, src: &Buffer, x: i32, y: i32) -> &mut Self {
        panic!("Not Implemented Yet!")
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

}