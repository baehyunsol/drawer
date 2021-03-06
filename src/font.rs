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


pub struct Font {
    size: usize,
    font_kit_font: FKFont,
    rendered_buffers: HashMap<u32, Buffer>,
    gap: Buffer,  // horizontal gap between characters
    color: Color,
    background: Option<Color>,
    outline: Option<Color>,
    underline: Option<Color>,
    letter_spacing: usize
}


impl Font {

    pub fn from_file(file: &str, size: usize, color: Color) -> Result<Font, ()> {

        let font_kit_font = get_font_from_file(file)?;

        let result = Font {
            font_kit_font, size, rendered_buffers: HashMap::new(),
            color, background: None, outline: None, underline: None, letter_spacing: size / 4,
            gap: Buffer::new(0, 0)
        };

        Ok(result)
    }

    pub fn from_bytes(bytes: Vec<u8>, size: usize, color: Color) -> Result<Font, ()> {

        let font_kit_font = get_font_from_bytes(bytes)?;

        let result = Font {
            font_kit_font, size, rendered_buffers: HashMap::new(),
            color, background: None, outline: None, underline: None, letter_spacing: size / 4,
            gap: Buffer::new(0, 0)
        };

        Ok(result)
    }

    pub fn init(&mut self) -> &mut Self {
        self.init_ascii();
        self.init_gap();
        self
    }

    pub fn init_ext(&mut self) -> &mut Self {
        self.rasterize_font(128, 512);
        self
    }

    pub fn set_background_color(&mut self, color: Option<Color>) -> &mut Self {
        self.background = color;
        self
    }

    pub fn set_outline(&mut self, outline: Option<Color>) -> &mut Self {
        self.outline = outline;
        self
    }

    pub fn set_underline(&mut self, underline: Option<Color>) -> &mut Self {
        self.underline = underline;
        self
    }

    pub fn set_letter_spacing(&mut self, letter_spacing: usize) -> &mut Self {
        self.letter_spacing = letter_spacing;
        self
    }

    fn init_ascii(&mut self) {
        self.rasterize_font(32, 127);
    }

    fn rasterize_font(&mut self, from: u32, to: u32) {

        let outline_width = match self.outline {
            None => {0}
            _ => {self.size / 72}
        };

        for i in from..to + 1 {
            match self.render_single_char(i) {
                Ok(mut buff) => {

                    buff = match get_raster_width(&self.font_kit_font, self.size, i) {
                        Ok(w) => buff.crop(0, 0, (w + outline_width) as i32, buff.height as i32),
                        _ => buff
                    };

                    self.rendered_buffers.insert(i, buff);
                }
                _ => {}
            }
        }

    }

    fn init_gap(&mut self) {

        let (_, height) = self.get_size(&" ".to_string());

        let mut result = Buffer::new(self.letter_spacing, height);

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
            curr_x += self.letter_spacing;
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

            curr += self.letter_spacing;
        }

        if string.len() > 0 {
            curr -= self.letter_spacing;
        }

        (curr, get_font_height(self.size))
    }

    fn render_single_char(&self, utf_code: u32) -> Result<Buffer, ()> {

        let (width, height) = (self.size, get_font_height(self.size));
        let mut result = Buffer::new(width, height);

        let glyph_id = get_glyph_id(&self.font_kit_font, utf_code)?;

        match self.background {
            Some(c) => {result.fill(c);}
            _ => {}
        }

        result = draw_font_buffer(
            result,
            &self.font_kit_font,
            width, height, self.size,
            glyph_id, self.color
        );

        match self.outline {
            Some(c) => {result.blit(&draw_font_outline(&self.font_kit_font, self.size, utf_code, c)?, 0, 0);}
            _ => {}
        }

        match self.underline {
            Some(c) => {result.draw_line(0, self.size as i32, self.size as i32, self.size as i32, self.size as i32 / 20 + 1, c);}
            _ => {}
        }

        Ok(result)
    }

}


fn draw_font_outline(raw_font: &FKFont, size: usize, utf_code: u32, color: Color) -> Result<Buffer, ()> {

    let (width, height) = (size, get_font_height(size));
    let mut font_buffer = Buffer::new(width, height);
    let mut outline_buffer = Buffer::new(width, height);

    let glyph_id = get_glyph_id(raw_font, utf_code)?;

    if size == 0 || utf_code == 32 {
        return Ok(outline_buffer);
    }

    font_buffer = draw_font_buffer(
        font_buffer,
        raw_font,
        width, height, size,
        glyph_id, Color::rgb(255, 255, 255)
    );

    for x in 1..width - 1 {

        for y in 1.. height - 1 {
            let a = font_buffer.get_pixel(x, y).a;

            if a == 0 {
                continue;
            }

            let a_top = font_buffer.get_pixel(x, y - 1).a;
            let a_bottom = font_buffer.get_pixel(x, y + 1).a;
            let a_left = font_buffer.get_pixel(x - 1, y).a;
            let a_right = font_buffer.get_pixel(x + 1, y).a;

            if a_top == 255 && a_bottom == 255 && a_left == 255 && a_right == 255 {
                continue;
            }

            outline_buffer.set_pixel(x, y, color);
            outline_buffer.draw_circle(x as i32, y as i32, size as i32 / 72, color);
        }

    }

    Ok(outline_buffer)
}


fn get_raster_width(raw_font: &FKFont, size: usize, utf_code: u32) -> Result<usize, ()> {

    if size == 0 {
        return Ok(0);
    }

    if utf_code == 32 {
        return Ok(size / 3 + 1)
    }

    let (width, height) = (size, get_font_height(size));
    let mut result = Buffer::new(width, height);

    let glyph_id = get_glyph_id(raw_font, utf_code)?;

    result = draw_font_buffer(
        result,
        raw_font,
        width, height, size,
        glyph_id, Color::rgb(255, 255, 255)
    );

    let mut curr = width - 1;

    while curr > 0 {

        for y in 0..height {

            if result.get_pixel(curr, y).a > 0 {
                return Ok(curr + 1);
            }

        }

        curr -= 1;
    }

    Ok(0)
}


fn get_glyph_id(raw_font: &FKFont, utf_code: u32) -> Result<u32, ()> {

    let character = match std::char::from_u32(utf_code) {
        None => {return Err(())},
        Some(c) => c
    };

    let glyph_id = match raw_font.glyph_for_char(character) {
        Some(id) => id,
        None => {return Err(())}
    };

    Ok(glyph_id)
}


fn draw_font_buffer(mut buffer: Buffer, raw_font: &FKFont, width: usize, height: usize, size: usize, glyph_id: u32, color: Color) -> Buffer {

    let mut canvas = Canvas::new(Vector2I::new(width as i32, height as i32), Format::A8);

    raw_font.rasterize_glyph(
        &mut canvas,
        glyph_id,
        size as f32,
        Transform2F::from_translation(Vector2F::new(0.0, size as f32)),
        HintingOptions::None,
        RasterizationOptions::GrayscaleAa,
    ).unwrap();

    for x in 0..width {
        for y in 0..height {
            if canvas.pixels[y * width + x] != 0 {
                buffer.set_pixel(x, y, color);
            }
        }
    }

    buffer
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
    size + size / 4
}


const QUESTION_MARK: u32 = 63;