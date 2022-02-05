mod buffer;
mod color;
mod transform;
mod draw;


fn main() {

    let mut b = buffer::Buffer::new(1080, 1080);
    let color1 = color::Color::rgb(128, 255, 128);
    b.draw_circle_line(560, 600, 320, 5, color1);
    b.draw_circle_line(360, 400, 70, 2, color1);
    b.save("test.png").unwrap();
}
