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

        else {
            panic!("what should i do here?")
        }

    }

}