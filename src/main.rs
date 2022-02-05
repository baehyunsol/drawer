mod buffer;
mod color;
mod transform;
mod draw;


fn main() {

    let mut b = buffer::Buffer::new(1080, 1080);
    let color1 = color::Color::rgba(128, 255, 128, 192);
    let color2 = color::Color::rgba(255, 128, 255, 192);

    b.draw_rect(500, 500, 80, 80, color1);
    b.draw_triangle(500, 500, 579, 500, 579, 579, color2);

    b.save("test.png").unwrap();
}
