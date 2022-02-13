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

        Color::rgba(
            ((self.r as u32 * (255 - other.a) as u32 + other.r as u32 * other.a as u32) / 255) as u8,
            ((self.g as u32 * (255 - other.a) as u32 + other.g as u32 * other.a as u32) / 255) as u8,
            ((self.b as u32 * (255 - other.a) as u32 + other.b as u32 * other.a as u32) / 255) as u8,
            self.a.max(other.a)
        )

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