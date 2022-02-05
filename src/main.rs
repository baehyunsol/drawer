mod buffer;
mod color;
mod transform;
mod draw;


fn main() {

    let mut b = buffer::Buffer::new(1080, 1080);
    let color1 = color::Color::rgba(128, 255, 128, 128);
    let color2 = color::Color::rgba(255, 128, 255, 255);

    for y in 0..8 {

        for x in 0..8 {
            b.draw_circle(x * 72 + 36 * (y % 2) + 216, y * 62 + 216, 72, color1);
        }

    }

    let mut m = buffer::Buffer::new(1080, 1080);
    m.draw_rect(400, 300, 240, 180, color2);
    b.mask(&m);

    b.save("test.png").unwrap();
}
