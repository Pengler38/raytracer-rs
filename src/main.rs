use image::{Rgb, RgbImage};

fn main() {
    let mut img = RgbImage::new(256, 256);

    for x in 0..=255 {
        for y in 0..=255 {
            img.put_pixel(x as u32, y as u32, hello_world_image(x, y));
        }
    }
    _ = img.save("./test.png");
}

fn hello_world_image(x: u8, y: u8) -> Rgb<u8> {
    let y_clamp = if y > x { x } else { y };
    Rgb([255 - x, y, x - y_clamp])
}
