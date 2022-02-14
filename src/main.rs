mod buffer;
mod color;
mod transform;
mod draw;
mod font;
mod effect;


fn main() {

    let mut parent = buffer::Buffer::new(1280, 1280);
    let mut f = font::Font::from_file("font.otf", 32, color::Color::rgb(255, 255, 255)).unwrap();
    f.set_underline(Some(color::Color::rgb(255, 0, 255))).set_background_color(Some(color::Color::rgb(44, 44, 99))).init();

    let rendered_text = f.render(&String::from(" ABCDEFG "));
    let small = rendered_text.resize(92, 19);
    let big = rendered_text.resize(368, 76);
    let odd = rendered_text.resize(400, 48);

    parent.blit(&rendered_text, 50, 200);
    parent.blit(&small, 50, 300);
    parent.blit(&big, 50, 400);
    parent.blit(&odd, 50, 500);

    parent.save("test.png");
}
