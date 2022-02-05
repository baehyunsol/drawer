#[derive(Clone, Copy)]
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

    // other on self
    pub fn blit(&self, other: &Color) -> Self {

        if other.a == 255 {
            *other
        }

        else if other.a == 0 {
            *self
        }

        else {
            Color::rgba(
                self.r / 2 + other.r / 2,
                self.g / 2 + other.g / 2,
                self.b / 2 + other.b / 2,
                self.a / 2 + other.a / 2
            )
        }

    }

    pub fn into_8bit(&self) -> Self {
        Color::rgb(
            ((self.r as u32 + 1) / 32 * 32).min(255) as u8,
            ((self.g as u32 + 1) / 32 * 32).min(255) as u8,
            ((self.b as u32 + 1) / 64 * 64).min(255) as u8,
        )
    }

    pub fn into_12bit(&self) -> Self {
        Color::rgb(
            ((self.r as u32 + 1) / 16 * 16).min(255) as u8,
            ((self.g as u32 + 1) / 16 * 16).min(255) as u8,
            ((self.b as u32 + 1) / 16 * 16).min(255) as u8,
        )
    }

}