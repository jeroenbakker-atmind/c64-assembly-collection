use crate::{colors::Color, image_container::Palette4BitmapImage, palette::Palette4};

#[test]
fn pal4_pixel_encoding() {
    let mut image = Palette4BitmapImage::new(
        16,
        16,
        Palette4 {
            colors: [Color::White, Color::Grey, Color::DarkGrey, Color::Black],
        },
    );

    image.set_pixel_palette_index(0, 0, 3);
    image.set_pixel_palette_index(1, 0, 2);
    image.set_pixel_palette_index(2, 0, 1);
    image.set_pixel_palette_index(3, 0, 0);

    assert_eq!(image.bitmap[0], 0b11100100);

    assert_eq!(image.get_pixel_palette_index(0, 0), 3);
    assert_eq!(image.get_pixel_palette_index(1, 0), 2);
    assert_eq!(image.get_pixel_palette_index(2, 0), 1);
    assert_eq!(image.get_pixel_palette_index(3, 0), 0);
}
