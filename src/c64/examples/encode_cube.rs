use std::collections::HashSet;

use c64::{
    image_container::{bit_char::BitCharImage, Image},
    image_io::read_png::read_png,
};

fn main() {
    let mut images = vec![];
    for image_number in 1..=100 {
        let image = read_png(format!("resources/render/001/{image_number:04}.png").as_str());
        // convert to BitCharImageContainer each char is encoded in a single u64
        let mut bit_char_image = BitCharImage::new(320 / 8, 200 / 8);

        for y in 0..200 {
            for x in 0..320 {
                let sx = x / 2;
                let sy = y / 2;
                let c = image.get_pixel_color(sx, sy);
                if c.r > 0x7F {
                    bit_char_image.set_pixel_color(x, y);
                }
            }
        }

        images.push(bit_char_image);
    }

    let mut unique = HashSet::<u64>::new();
    for (index, image) in images.iter().enumerate() {
        unique.extend(image.chars.iter());
        println!("{index}: unique_chars={}, tot={}", image.count_unique(), unique.len());
    }
}
