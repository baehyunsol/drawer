#[derive(Copy, Clone, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

impl Color {

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color {r, g, b, a: 255}
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color {r, g, b, a}
    }

    pub fn blit(&self, other: Color) -> Color {

        if other.a == 255 {
            Color::rgb(
                other.r,
                other.g,
                other.b
            )
        }

        else {
            Color::rgba(
                ((self.r as u32 * self.a as u32 * (255 - other.a as u32) + other.r as u32 * other.a as u32 * 255) / 65025) as u8,
                ((self.g as u32 * self.a as u32 * (255 - other.a as u32) + other.g as u32 * other.a as u32 * 255) / 65025) as u8,
                ((self.b as u32 * self.a as u32 * (255 - other.a as u32) + other.b as u32 * other.a as u32 * 255) / 65025) as u8,
                ((self.a as u32 * (255 - other.a as u32) + 255 * other.a as u32) / 255) as u8
            )
        }

    }

}