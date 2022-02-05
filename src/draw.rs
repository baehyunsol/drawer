use crate::buffer::Buffer;
use crate::color::Color;


impl Buffer {

    pub fn fill(&mut self, color: Color) -> &mut Self {
        self.pixels = vec![vec![color; self.width]; self.height];
        self
    }

    pub fn draw_circle_line(&mut self, x: i32, y: i32, r: i32, width: i32, color: Color) -> &mut Self {

        let mut points = (neg_half(width)..pos_half(width)).map(
            |w|
            self.get_circle_line(x, y, r + w)
        ).collect::<Vec<Vec<(usize, usize)>>>().concat();

        points.sort_by_key(|p| p.0);
        points.dedup();

        for p in points.iter() {
            self.blit_pixel(p.0, p.1, color);
        }

        self
    }

    fn get_circle_line(&self, x: i32, y: i32, r: i32) -> Vec<(usize, usize)> {

        if r <= 0 {
            return vec![];
        }

        let mut curr_x = x;
        let mut curr_y = y + r;
        let mut points = Vec::with_capacity(r as usize * 6);

        while curr_x < x + r {

            while (x - curr_x) * (x - curr_x) + (y - curr_y) * (y - curr_y) >= r * r {
                push_symmetrical_points(curr_x, curr_y, x, y, &mut points);
                curr_y -= 1;
            }

            push_symmetrical_points(curr_x, curr_y, x, y, &mut points);
            curr_x += 1;
        }

        while curr_y > y {
            push_symmetrical_points(curr_x, curr_y, x, y, &mut points);
            curr_y -= 1;
        }

        points.push((x + r, y));
        points.push((x - r, y));

        points.into_iter().filter(
            |p|
            p.0 >= 0 && p.1 >= 0 && p.0 < self.width as i32 && p.1 < self.height as i32
        ).map(
            |p|
            (p.0 as usize, p.1 as usize)
        ).collect()
    }

    pub fn draw_circle(&mut self, x: i32, y: i32, r: i32, color: Color) -> &mut Self {

        if r <= 0 {
            return self;
        }

        let x_start = if x > r { x - r } else { 0 };
        let y_start = if y > r { y - r } else { 0 };
        let x_end = if x + r < self.width as i32 { x + r } else { self.width as i32 };
        let y_end = if y + r < self.height as i32 { y + r } else { self.height as i32 };

        let rr = r * r;

        for curr_x in x_start..x_end {

            for curr_y in y_start..y_end {

                if (curr_x - x) * (curr_x - x) + (curr_y - y) * (curr_y - y) < rr {
                    self.blit_pixel(curr_x as usize, curr_y as usize, color);
                }

            }

        }

        self
    }

    pub fn draw_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: Color) -> &mut Self {

        if w <= 0 || h <= 0 {
            return self;
        }

        let x_start = x.max(0);
        let y_start = y.max(0);
        let x_end = if x + w < self.width as i32 { x + w } else { self.width as i32 };
        let y_end = if y + h < self.height as i32 { y + h } else { self.height as i32 };

        for curr_x in x_start..x_end {

            for curr_y in y_start..y_end {
                self.blit_pixel(curr_x as usize, curr_y as usize, color);
            }

        }

        self
    }

    pub fn draw_rect_line(&mut self, x: i32, y: i32, w: i32, h: i32, width: i32, color: Color) -> &mut Self {

        self.draw_line(x + neg_half(width), y, x + w + pos_half(width), y, width, color);
        self.draw_line(x + neg_half(width), y + h, x + w + pos_half(width), y + h, width, color);
        self.draw_line(x, y, x, y + h, width, color);
        self.draw_line(x + w, y, x + w, y + h, width, color);

        self
    }

    pub fn draw_triangle(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, color: Color) -> &mut Self {
        panic!("Not Implemented!")
    }

    pub fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, width: i32, color: Color) -> &mut Self {

        if width <= 0 {
            return self;
        }

        let is_horizontal = (x1 - x2).abs() > (y1 - y2).abs();

        if is_horizontal {

            for offset in neg_half(width)..pos_half(width) {
                self._draw_line(x1, y1 + offset, x2, y2 + offset, color);
            }

        }

        else {

            for offset in neg_half(width)..pos_half(width) {
                self._draw_line(x1 + offset, y1, x2 + offset, y2, color);
            }

        }

        self
    }

    fn _draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: Color) -> &mut Self {

        if x1 == x2 {
            let begin = y1.min(y2).max(0) as usize;
            let end = y1.max(y2).min(self.height as i32) as usize;

            for y in begin..end {
                self.blit_pixel(x1 as usize, y, color);
            }

        }

        else if y1 == y2 {
            let begin = x1.min(x2).max(0) as usize;
            let end = x1.max(x2).min(self.width as i32) as usize;

            for x in begin..end {
                self.blit_pixel(x, y1 as usize, color);
            }

        }

        else {
            let is_horizontal = (x1 - x2).abs() > (y1 - y2).abs();

            if is_horizontal {
                let (mut curr_x, begin_x, dest_x, diff, begin_y) = if x1 < x2 {
                    (x1, x1, x2, y2 - y1, y1)
                } else {
                    (x2, x2, x1, y1 - y2, y2)
                };

                while curr_x < dest_x {
                    let curr_y = begin_y +(curr_x - begin_x) * diff / (dest_x - begin_x);
                    self.blit_pixel(curr_x as usize, curr_y as usize, color);
                    curr_x += 1;
                }

            }

            else {
                let (mut curr_y, begin_y, dest_y, diff, begin_x) = if y1 < y2 {
                    (y1, y1, y2, x2 - x1, x1)
                } else {
                    (y2, y2, y1, x1 - x2, x2)
                };

                while curr_y < dest_y {
                    let curr_x = begin_x + (curr_y - begin_y) * diff / (dest_y - begin_y);
                    self.blit_pixel(curr_x as usize, curr_y as usize, color);
                    curr_y += 1;
                }

            }

        }

        self
    }

}


fn push_symmetrical_points(x: i32, y: i32, c_x: i32, c_y: i32, vec: &mut Vec<(i32, i32)>) {

    vec.push((x, y));
    vec.push((2 * c_x - x, y));
    vec.push((2 * c_x - x, 2 * c_y - y));
    vec.push((x, 2 * c_y - y));
}


fn neg_half(n: i32) -> i32 {
    - n / 2
}

fn pos_half(n: i32) -> i32 {
    (n + 1) / 2
}