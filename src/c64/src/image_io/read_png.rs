use std::fs::File;

use png::ColorType;

use crate::image_container::DefaultImageContainer;

pub fn read_png(input_file_path: &str) -> DefaultImageContainer {
    let decoder = png::Decoder::new(File::open(input_file_path).unwrap());
    let mut reader = decoder.read_info().unwrap();
    // Allocate the output buffer.
    let mut buf = vec![0; reader.output_buffer_size()];
    // Read the next frame. An APNG might contain multiple frames.
    let info = reader.next_frame(&mut buf).unwrap();

    let components_per_pixel = if info.color_type == ColorType::Rgba { 4 } else { 3 };

    DefaultImageContainer {
        width: info.width as usize,
        height: info.height as usize,

        buffer: buf.clone(),
        components_per_pixel,
    }
}
