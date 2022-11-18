
extern crate clap;

use clap::{Parser, ValueEnum};
use std::fs::File;
use std::io::Write;
use c64::image_converter::{DefaultImageContainer, ImageConverter, StandardCharacterMode};

#[derive(Debug, Copy, Clone, ValueEnum)]
enum ConversionFormat {
    StandardText,
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

    /// Prefix to add to generated variable names to make them unique.
    #[arg(short, long)]
    variable_prefix: String,
 }
 
 
fn main() {
    let args = Arguments::parse();

    let decoder = png::Decoder::new(File::open(args.input_filename).unwrap());
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
    let converter = StandardCharacterMode::default();

    let text_image = converter.convert(&image);

    let mut writer = File::create(args.output_filename).unwrap();
     
    writer.write_all(format!("{}_chars:", args.variable_prefix).as_bytes()).unwrap();
    for chunk in text_image.characters.chunks(16) {
        print!("  .byte ");
        for (i, a) in chunk.iter().enumerate() {
            if i != 0 {
                print!(", ");
            }
            print!("${:2x}", a);
        }
        println!()
    }
    println!("{}_colors:", args.variable_prefix);
    for chunk in text_image.foreground_colors.chunks(16) {
        print!("  .byte ");
        for (i, a) in chunk.iter().enumerate() {
            if i != 0 {
                print!(", ");
            }
            print!("${:02x}", u8::from(*a));
        }
        println!()
    }
    println!("{}_background:", args.variable_prefix);
    println!("  .byte ${:02x}", u8::from(text_image.background_color));
     
}