mod buffer;
mod color;
mod transform;
mod draw;
mod font;
mod effect;


fn main() {

    let mut parent = buffer::Buffer::new(1280, 1280);
    let mut sample = buffer::Buffer::load("sample.png").unwrap();

    sample.filter(0, 0, 1280, 1280, Box::new(|c: &color::Color| c.into_12bit()));

    parent.blit(&sample, 0, 0);

    parent.save("test.png");
}
