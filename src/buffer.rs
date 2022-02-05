use crate::color::Color;


#[derive(Clone)]
pub struct Buffer {
    pub pixels: Vec<Vec<Color>>,
    pub width: usize,
    pub height: usize
}


impl Buffer {

    pub fn new(width: usize, height: usize) -> Buffer {
        Buffer {
            pixels: vec![vec![Color::rgba(0, 0, 0, 0); width]; height],
            width, height
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        self.pixels[y][x]
    }

    pub fn blit_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.pixels[y][x] = self.pixels[y][x].blit(&color)
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.pixels[y][x] = color;
    }

    pub fn save(&self, file: &str) -> Result<(), image::ImageError> {

        let mut result = image::ImageBuffer::new(self.width as u32, self.height as u32);

        for x in 0..self.width {

            for y in 0..self.height {
                let Color{r, g, b, a} = self.pixels[y][x];
                result.put_pixel(x as u32, y as u32, image::Rgba([r, g, b, a]));
            }

        }

        result.save(file)
    }

    pub fn load(file: &str) -> Result<Buffer, image::ImageError> {

        let im = image::io::Reader::open(file)?.decode()?.to_rgba8();

        let w = im.width();
        let h = im.height();
        let mut result = Buffer::new(w as usize, h as usize);

        for x in 0..w {

            for y in 0..h {
                let [r, g, b, a] = im.get_pixel(x, y).0;
                result.set_pixel(x as usize, y as usize, Color::rgba(r, g, b, a));
            }

        }

        Ok(result)
    }

}