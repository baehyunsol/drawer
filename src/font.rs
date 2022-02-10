use crate::color::Color;
use crate::buffer::Buffer;
use font_kit::canvas::{Canvas, RasterizationOptions, Format};
use font_kit::hinting::HintingOptions;
use font_kit::font::Font as FKFont;
use pathfinder_geometry::vector::{Vector2I, Vector2F};
use pathfinder_geometry::transform2d::Transform2F;
use std::sync::Arc;
use std::fs::File;
use std::collections::HashMap;

/*
TODO

make fonts more horizontally tight
*/


pub struct Font {
    size: usize,
    font_kit_font: FKFont,
    rendered_buffers: HashMap<u32, Buffer>,
    gap: Buffer,  // horizontal gap between characters
    color: Color,
    background: Option<Color>,
    outline: Option<Color>,
    underline: Option<Color>,
    horizontal_margin: usize
}


impl Font {

    pub fn from_file(file: &str, size: usize, color: Color) -> Result<Font, ()> {

        let font_kit_font = get_font_from_file(file)?;

        let mut result = Font {
            font_kit_font, size, rendered_buffers: HashMap::new(),
            color, background: None, outline: None, underline: None, horizontal_margin: size / 5,
            gap: Buffer::new(0, 0)
        };

        Ok(result)
    }

    pub fn init(&mut self) -> &mut Self {
        self.init_ascii();
        self.init_gap();
        self
    }

    pub fn set_background_color(&mut self, color: Option<Color>) -> &mut Self {
        self.background = color;
        self
    }

    pub fn set_underline(&mut self, underline: Option<Color>) -> &mut Self {
        self.underline = underline;
        self
    }

    fn init_ascii(&mut self) {

        for i in 32..127 {
            self.rendered_buffers.insert(i, self.render_single_char(i));
        }

    }

    fn init_gap(&mut self) {

        let (_, height) = self.get_size(&" ".to_string());

        let mut result = Buffer::new(self.horizontal_margin, height);

        match self.background {
            Some(c) => {result.fill(c);}
            _ => {}
        }

        match self.underline {
            Some(c) => {result.draw_line(0, self.size as i32, self.size as i32, self.size as i32, self.size as i32 / 20 + 1, c);}
            _ => {}
        }

        self.gap = result;
    }

    pub fn render(&self, string: &String) -> Buffer {

        let (width, height) = self.get_size(&string);
        let string = string.encode_utf16().collect::<Vec<u16>>();

        let mut result = Buffer::new(width, height);
        let mut curr_x: usize = 0;

        for c in string.iter() {
            let curr_buf = match self.rendered_buffers.get(&(*c as u32)) {
                None => self.rendered_buffers.get(&QUESTION_MARK).unwrap(),
                Some(b) => b
            };

            result.blit(curr_buf, curr_x as i32, 0);
            curr_x += curr_buf.width;
            result.blit(&self.gap, curr_x as i32, 0);
            curr_x += self.horizontal_margin;
        }

        result
    }

    pub fn get_size(&self, string: &String) -> (usize, usize) {

        let string = string.encode_utf16().collect::<Vec<u16>>();
        let mut curr = 0;

        for c in string.iter() {
            match self.rendered_buffers.get(&(*c as u32)) {
                None => {curr += self.rendered_buffers.get(&QUESTION_MARK).unwrap().width;}
                Some(b) => {curr += b.width;}
            }

            curr += self.horizontal_margin;
        }

        if string.len() > 0 {
            curr -= self.horizontal_margin;
        }

        (curr, get_font_height(self.size))
    }


    fn render_single_char(&self, glyph: u32) -> Buffer {
    
        let glyph = match std::char::from_u32(glyph) {
            None => '?',
            Some(c) => c
        };
    
        let invalid_glyph_id = self.font_kit_font.glyph_for_char('?').unwrap();
    
        let (width, height) = (self.size, get_font_height(self.size));
    
        let mut result = Buffer::new(width, height);

        match self.background {
            Some(c) => {result.fill(c);}
            _ => {}
        }
    
        let glyph_id = match self.font_kit_font.glyph_for_char(glyph) {
            Some(id) => id,
            None => invalid_glyph_id
        };
    
        let mut canvas = Canvas::new(Vector2I::new(width as i32, height as i32), Format::A8);
    
        self.font_kit_font.rasterize_glyph(
            &mut canvas,
            glyph_id,
            self.size as f32,
            Transform2F::from_translation(Vector2F::new(0.0, self.size as f32)),
            HintingOptions::None,
            RasterizationOptions::GrayscaleAa,
        )
        .unwrap();
    
        for x in 0..width {
            for y in 0..height {
                if canvas.pixels[y * width + x] != 0 {
                    result.set_pixel(x, y, Color::rgb(255, 255, 255));
                }
            }
        }

        match self.underline {
            Some(c) => {result.draw_line(0, self.size as i32, self.size as i32, self.size as i32, self.size as i32 / 20 + 1, c);}
            _ => {}
        }
    
        result
    }

}


fn get_font_from_file(file: &str) -> Result<FKFont, ()> {

    match File::open(file) {
        Err(_) => Err(()),
        Ok(mut f) => match FKFont::from_file(&mut f, 0) {
            Err(_) => Err(()),
            Ok(f) => Ok(f)
        }
    }

}


fn get_font_from_bytes(bytes: Vec<u8>) -> Result<FKFont, ()> {

    match FKFont::from_bytes(Arc::new(bytes), 0) {
        Err(_) => Err(()),
        Ok(f) => Ok(f)
    }

}


fn get_font_height(size: usize) -> usize {
    size + size / 5
}


const QUESTION_MARK: u32 = 63;