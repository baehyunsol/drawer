mod buffer;
mod color;
mod transform;
mod draw;
mod font;


fn main() {

    let mut parent = buffer::Buffer::new(1280, 1280);
    let mut f = font::Font::from_file("font.otf", 32, color::Color::rgb(255, 255, 255)).unwrap();
    f.set_underline(Some(color::Color::rgb(255, 255, 255))).set_background_color(Some(color::Color::rgb(44, 44, 99))).init();
    parent.blit(&f.render(&String::from("Hello World!!! 볼빨간사춘기")), 0, 0);

    let c = color::Color::rgba(44, 44, 199, 254);
    parent.draw_ellipse(640, 640, 8, 3, 32, c);

    parent.save("test.png");
}
