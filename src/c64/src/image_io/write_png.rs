use std::{fs::File, io::BufWriter, path::Path};

use crate::image_container::Image;

pub fn write_png(output_path: &str, image: &dyn Image) {
    let path = Path::new(output_path);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, image.width() as u32, image.height() as u32); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
    encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2)); // 1.0 / 2.2, unscaled, but rounded
    let source_chromaticities = png::SourceChromaticities::new(
        // Using unscaled instantiation here
        (0.31270, 0.32900),
        (0.64000, 0.33000),
        (0.30000, 0.60000),
        (0.15000, 0.06000),
    );
    encoder.set_source_chromaticities(source_chromaticities);
    let mut writer = encoder.write_header().unwrap();
    let mut data = Vec::<u8>::with_capacity(image.width() * image.height() * 3);
    for y in 0..image.height() {
        for x in 0..image.width() {
            let color = image.get_pixel_color(x, y);
            data.push(color.r);
            data.push(color.g);
            data.push(color.b);
        }
    }

    writer.write_image_data(&data).unwrap();
}
