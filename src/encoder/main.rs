use std::{convert, fs::File};

use c64::{
    colors::Color,
    image_container::{DefaultImageContainer, Palette4BitmapImage},
    image_converter::palette_bitmap::convert_to_palette4,
    palette::Palette4,
};
use clap::Parser;
use png::ColorType;

fn main() {
    let args = Arguments::parse();

    let decoder = png::Decoder::new(File::open(&args.input_folder).unwrap());
    let mut reader = decoder.read_info().unwrap();
    // Allocate the output buffer.
    let mut buf = vec![0; reader.output_buffer_size()];
    // Read the next frame. An APNG might contain multiple frames.
    let info = reader.next_frame(&mut buf).unwrap();

    let components_per_pixel = if info.color_type == ColorType::Rgba {
        4
    } else {
        3
    };

    let image = DefaultImageContainer {
        width: info.width as usize,
        height: info.height as usize,

        buffer: buf.clone(),
        components_per_pixel,
    };

    let palette = Palette4 {
        colors: [Color::White, Color::Black, Color::Grey, Color::Purple],
    };

    let image_pal4 = convert_to_palette4(&image, palette);
    println!("{image_pal4:?}");
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    /// Input folder containing PNG files to encode. Width and height of the image must be divisable by 8.
    #[arg(short, long)]
    input_folder: String,

    /// Output file to store the converted image.
    #[arg(short, long)]
    output_filename: String,
}
