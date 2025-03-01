use c64::{image_container::SRGBImageContainer, image_io::write_png::write_png};
use c64_colors::colors::SRGB;
use c64_dithering::dithering::bayer::{Bayer4x4, DitherPattern};

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

fn main() {
    let pattern = Bayer4x4 {};
    let mut colors = vec![];
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let threshold = pattern.threshold(x, y);
            colors.push(SRGB {
                r: threshold,
                g: threshold,
                b: threshold,
            });
        }
    }
    let bayer_image = SRGBImageContainer {
        width: WIDTH,
        height: HEIGHT,
        buffer: colors,
    };
    write_png("resources/bayer_4x4.png", &bayer_image);
}
