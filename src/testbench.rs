use crate::color::Color;

pub fn random(mut seed: usize) -> u32 {

    if seed < 1024 {
        seed *= seed;
    }

    let mut seed = seed as u32;

    for _ in 0..3 {
        seed = ((seed % 21 + seed % 23 + seed % 25) * 821 + (seed % 27 + seed % 29 + seed % 31) * 823 + (seed % 33 + seed % 35 + seed % 37) * 827 + (seed % 39 + seed % 41 + seed % 43) * 829) % 65536;
    }

    seed
}

pub fn rand_pixel(seed: usize) -> Color {

    let seed1 = random(seed * 2);
    let seed2 = random(seed * 2 + 1);

    Color::rgba(
        (seed1 % 256) as u8,
        (seed1 / 256) as u8,
        (seed2 % 256) as u8,
        (seed2 / 512 + 192).min(255) as u8,
    )
}

mod tests {

    #[test]
    fn file_io_test() {
        use crate::buffer::Buffer;
        use super::rand_pixel;

        let mut buff = Buffer::new(512, 512);

        for x in 0..512 {

            for y in 0..512 {
                buff.set_pixel(x, y, rand_pixel(x * 512 + y));
            }

        }

        buff.save("test.bmp").unwrap();

        let comp = Buffer::load("test.bmp").unwrap();

        assert!(buff == comp);
    }

}