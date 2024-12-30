use c64::{
    image_converter::palette_bitmap::convert_to_palette4,
    image_io::{read_png::read_png, write_png::write_png},
};
use c64_colors::{colors::Color, palette::Palette4};
use c64_dithering::dithering::no_dithering::NoDithering;
use clap::Parser;

fn main() {
    let args = Arguments::parse();

    let palette = Palette4 {
        colors: [Color::White, Color::Black, Color::Grey, Color::Purple],
    };
    let dithering = NoDithering {};

    let image = read_png(&args.input_folder);
    let image_pal4 = convert_to_palette4(&image, palette, &dithering);

    println!("{image_pal4:?}");
    // write_png("test.png", &image_pal4);
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
