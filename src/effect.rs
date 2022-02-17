use crate::buffer::Buffer;
use crate::color::Color;

impl Buffer {

    pub fn filter(&mut self, x: usize, y: usize, w: usize, h: usize, f: Box<dyn Fn(&Color) -> Color>) -> &mut Self {

        if x >= self.width || y >= self.height || w == 0 || h == 0 {
            return self;
        }

        for curr_x in x..(x + w).min(self.width) {

            for curr_y in y..(y + h).min(self.height) {
                self.set_pixel(curr_x, curr_y, f(&self.get_pixel(curr_x, curr_y)));
            }
        
        }

        self
    }

    pub fn blur(&mut self, x: usize, y: usize, w: usize, h: usize, intensity: usize) -> &mut Self {

        if x >= self.width || y >= self.height || w == 0 || h == 0 || intensity == 0 {
            return self;
        }

        let intensity = intensity as i32;

        for curr_x in x..(x + w).min(self.width) {

            for curr_y in y..(y + h).min(self.height) {
                let mut color_count = 0.0;
                let [mut r, mut g, mut b, mut a] = [0.0;4];

                for dx in -intensity .. intensity + 1 {

                    if (dx + curr_x as i32) < 0 || (dx + curr_x as i32) >= self.width as i32 {
                        continue;
                    }

                    for dy in -intensity .. intensity + 1 {

                        if (dy + curr_y as i32) < 0 || (dy + curr_y as i32) >= self.height as i32 {
                            continue;
                        }

                        let curr_color = self.get_pixel((dx + curr_x as i32) as usize, (dy + curr_y as i32) as usize);
                        color_count += 1.0;
                        r += curr_color.r as f32;
                        g += curr_color.g as f32;
                        b += curr_color.b as f32;
                        a += curr_color.a as f32;
                    }

                }

                self.set_pixel(
                    curr_x, curr_y,
                    Color::rgba(
                        (r / color_count) as u8,
                        (g / color_count) as u8,
                        (b / color_count) as u8,
                        (a / color_count) as u8,
                    )
                );
            }

        }

        self
    }

}
