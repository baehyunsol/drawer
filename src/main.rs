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

    parent.blit(&f.render(&String::from(" Hello World!!! 볼빨간사춘기 ")), 0, 0);
    parent.blit(&f.render(&String::from(" ABCDEFG... Really? gj <- below line! ")), 0, 100);
    parent.blit(&f.render(&String::from(" ABCDEFG... Really? gj <- below line! ")), 0, 200);
    parent.blit(&f.render(&String::from(" 0xbad_c0ffee_bad_f00d is 13464654573481095181 ")), 0, 300);

    let c = color::Color::rgba(44, 44, 199, 192);
    parent.draw_ellipse(640, 300, 8, 3, 32, c);

    parent.blur(0, 0, 640, 640, 4);

    parent.save("test.png");
}
