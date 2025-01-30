use c64::{image_container::SRGBImageContainer, image_io::write_png::write_png};
use c64_colors::colors::SRGB;
use c64_dithering::dithering::bayer::{Bayer2x2, DitherPattern};

fn main() {
    let pattern = Bayer2x2 {};
    let mut colors = vec![];
    for y in 0..100 {
        for x in 0..160 {
            let threshold = pattern.threshold(x, y);
            colors.push(SRGB {
                r: threshold,
                g: threshold,
                b: threshold,
            });
        }
    }
    let bayer_image = SRGBImageContainer {
        width: 160,
        height: 100,
        buffer: colors,
    };
    write_png("bayer_2x2.png", &bayer_image);
}
