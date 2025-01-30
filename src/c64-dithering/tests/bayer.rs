use c64_colors::colors::{Color, SRGB};
use c64_dithering::dithering::{
    bayer::{Bayer2x2, Bayer4x4, ThresholdDithering},
    Dithering,
};

#[test]
fn gradient_2x2() {
    let bayer = ThresholdDithering {
        color_dark: SRGB::from(Color::Black),
        color_bright: SRGB::from(Color::White),
        dither_pattern: Bayer2x2 {},
    };

    for y in 0..8 {
        for x in 0..64 {
            let color_in = SRGB::from_rgb(x as u8 * 4, x as u8 * 4, x as u8 * 4);
            let color = bayer.dither(x, y, color_in);
            if color.r > 0 {
                print!("#")
            } else {
                print!(" ")
            }
        }
        println!("");
    }
}

#[test]
fn gradient_4x4() {
    let bayer = ThresholdDithering {
        color_dark: SRGB::from(Color::Black),
        color_bright: SRGB::from(Color::White),
        dither_pattern: Bayer4x4 {},
    };

    for y in 0..8 {
        for x in 0..64 {
            let color_in = SRGB::from_rgb(x as u8 * 4, x as u8 * 4, x as u8 * 4);
            let color = bayer.dither(x, y, color_in);
            if color.r > 0 {
                print!("#")
            } else {
                print!(" ")
            }
        }
        println!("");
    }
}
