extern crate clap;

use c64::image_container::{difference, Image};
use c64::image_converter::{ConversionQuality, ImageConverter, StandardBitmapMode, StandardCharacterMode};
use c64::image_io::read_png::read_png;
use c64_colors::colors::Color;
use clap::{Parser, ValueEnum};
use std::fs::File;
use std::io::Write;

#[derive(Debug, Copy, Clone, ValueEnum)]
enum ConversionFormat {
    /// Convert image to be used on C64 standard text mode.
    StandardText,
    /// Convert image to be used on C64 standard text mode, generating a custom charset.
    StandardTextCustomCharset,
    /// Convert image to be used on C64 standard bitmap mode.
    StandardBitmap,
}

#[derive(Debug, Copy, Clone, ValueEnum)]
enum OutputEncoding {
    /// Store the result to contain raw data that can be directly interpreted by the VIC chip, eg RAW memory.
    Asm,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    /// Input PNG file to be converted. Width and height of the image must be divisable by 8.
    #[arg(short, long)]
    input_filename: String,

    /// Output file to store the converted image.
    #[arg(short, long)]
    output_filename: String,

    /// Type of conversion to perform.
    #[arg(long, value_enum)]
    format: ConversionFormat,

    #[arg(long)]
    output_encoding: OutputEncoding,

    /// Prefix to add to generated variable names to make them unique.
    #[arg(long)]
    output_variable_prefix: String,
}

fn main() {
    let args = Arguments::parse();

    let image = read_png(&args.input_filename);

    match args.format {
        ConversionFormat::StandardText => convert_standard_text(&args, &image),
        ConversionFormat::StandardTextCustomCharset => convert_standard_text_custom_char_set(&args, &image),
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
    write_asm_bytes(&mut writer, &text_image.characters);

    writer
        .write_all(format!("{}_colors:\n", args.output_variable_prefix).as_bytes())
        .unwrap();
    write_asm_colors(&mut writer, &text_image.foreground_colors);

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

fn convert_standard_text_custom_char_set(args: &Arguments, image: &dyn Image) {
    let converter = StandardCharacterMode {
        quality: ConversionQuality::CustomCharAndColor,
        ..StandardCharacterMode::default()
    };
    let text_image = converter.convert(image);
    let diff: usize = difference(image, &text_image);
    println!("{} chars, difference {}", text_image.characters.len(), diff);

    let mut writer = File::create(&args.output_filename).unwrap();
    writer
        .write_all(format!("{}_chars:\n", args.output_variable_prefix).as_bytes())
        .unwrap();
    write_asm_bytes(&mut writer, &text_image.characters);

    writer
        .write_all(format!("{}_colors:\n", args.output_variable_prefix).as_bytes())
        .unwrap();
    write_asm_colors(&mut writer, &text_image.foreground_colors);

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

    writer
        .write_all(format!("{}_charset:\n", args.output_variable_prefix).as_bytes())
        .unwrap();
    let charset_bytes = Vec::<u8>::from(text_image.charset);
    write_asm_bytes(&mut writer, &charset_bytes);
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
    write_asm_bytes(&mut writer, &bitmap_image.bitmap);

    writer
        .write_all(format!("{}_colors:\n", args.output_variable_prefix).as_bytes())
        .unwrap();
    write_asm_bytes(&mut writer, &bitmap_image.colors);
}

fn write_asm_bytes(writer: &mut File, bytes: &[u8]) {
    for chunk in bytes.chunks(16) {
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

fn write_asm_colors(writer: &mut File, colors: &[Color]) {
    let bytes: Vec<u8> = colors.iter().map(|c| u8::from(*c)).collect();
    write_asm_bytes(writer, &bytes);
}
