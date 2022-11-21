extern crate clap;

use c64::image_container::{difference, DefaultImageContainer, Image};
use c64::image_converter::{
    ConversionQuality, ImageConverter, StandardBitmapMode, StandardCharacterMode,
};
use clap::{Parser, ValueEnum};
use std::fs::File;
use std::io::Write;

#[derive(Debug, Copy, Clone, ValueEnum)]
enum ConversionFormat {
    StandardText,
    StandardBitmap,
}

#[derive(Debug, Copy, Clone, ValueEnum)]
enum OutputEncoding {
    Asm,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    /// Input PNG file to be converted.
    #[arg(short, long)]
    input_filename: String,

    /// Output file containing the converted output.
    #[arg(short, long)]
    output_filename: String,

    /// Type of conversion to do.
    #[arg(short, long, value_enum)]
    format: ConversionFormat,

    #[arg(short, long)]
    output_encoding: OutputEncoding,

    /// Prefix to add to generated variable names to make them unique.
    #[arg(short, long)]
    output_variable_prefix: String,
}

fn main() {
    let args = Arguments::parse();

    let decoder = png::Decoder::new(File::open(&args.input_filename).unwrap());
    let mut reader = decoder.read_info().unwrap();
    // Allocate the output buffer.
    let mut buf = vec![0; reader.output_buffer_size()];
    // Read the next frame. An APNG might contain multiple frames.
    let info = reader.next_frame(&mut buf).unwrap();

    let image = DefaultImageContainer {
        width: info.width as usize,
        height: info.height as usize,

        buffer: buf.clone(),
        components_per_pixel: 4,
    };

    match args.format {
        ConversionFormat::StandardText => convert_standard_text(&args, &image),
        ConversionFormat::StandardBitmap => convert_standard_bitmap(&args, &image),
    };
}

fn convert_standard_text(args: &Arguments, image: &dyn Image) {
    let converter = StandardCharacterMode {
        quality: ConversionQuality::EachCharAndColor,
        ..StandardCharacterMode::default()
    };
    let text_image = converter.convert(image);
    let diff: usize = difference(image, &text_image);
    println!("{} chars, difference {}", text_image.characters.len(), diff);

    let mut writer = File::create(&args.output_filename).unwrap();
    writer
        .write_all(format!("{}_chars:\n", args.output_variable_prefix).as_bytes())
        .unwrap();
    for chunk in text_image.characters.chunks(16) {
        let mut line = String::new();
        line += "  .byte ";
        for (i, a) in chunk.iter().enumerate() {
            if i != 0 {
                line += ", ";
            }
            line += &format!("${:02x}", a);
        }
        line += "\n";
        writer.write_all(line.as_bytes()).unwrap();
    }

    writer
        .write_all(format!("{}_colors:\n", args.output_variable_prefix).as_bytes())
        .unwrap();
    for chunk in text_image.foreground_colors.chunks(16) {
        let mut line = String::new();
        line += "  .byte ";
        for (i, a) in chunk.iter().enumerate() {
            if i != 0 {
                line += ", ";
            }
            line += &format!("${:02x}", u8::from(*a));
        }
        line += "\n";
        writer.write_all(line.as_bytes()).unwrap();
    }
    writer
        .write_all(
            format!(
                "{}_background:\n  .byte ${:02x}\n",
                args.output_variable_prefix,
                u8::from(text_image.background_color)
            )
            .as_bytes(),
        )
        .unwrap();
}

fn convert_standard_bitmap(args: &Arguments, image: &dyn Image) {
    let converter = StandardBitmapMode {
        ..StandardBitmapMode::default()
    };
    let bitmap_image = converter.convert(image);

    let mut writer = File::create(&args.output_filename).unwrap();
    writer
        .write_all(format!("{}_bitmap:\n", args.output_variable_prefix).as_bytes())
        .unwrap();
    for chunk in bitmap_image.bitmap.chunks(16) {
        let mut line = String::new();
        line += "  .byte ";
        for (i, a) in chunk.iter().enumerate() {
            if i != 0 {
                line += ", ";
            }
            line += &format!("${:02x}", a);
        }
        line += "\n";
        writer.write_all(line.as_bytes()).unwrap();
    }

    writer
        .write_all(format!("{}_colors:\n", args.output_variable_prefix).as_bytes())
        .unwrap();
    for chunk in bitmap_image.colors.chunks(16) {
        let mut line = String::new();
        line += "  .byte ";
        for (i, a) in chunk.iter().enumerate() {
            if i != 0 {
                line += ", ";
            }
            line += &format!("${:02x}", u8::from(*a));
        }
        line += "\n";
        writer.write_all(line.as_bytes()).unwrap();
    }
}
