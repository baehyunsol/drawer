mod buffer;
mod color;
mod transform;
mod draw;
mod font;
mod effect;


fn main() {

    let mut test = buffer::Buffer::new(1440, 1440);

    let mut font1 = font::Font::from_file("font.otf", 96, color::Color::rgb(192, 44, 44)).unwrap();
    font1.set_outline(Some(color::Color::rgb(222, 222, 222)));
    font1.init();

    let rr = font1.render(&String::from("ABCDEFG This is a sample text!"));
    test.blit(&rr, 0, 400);

    test.save("test.png");
}
