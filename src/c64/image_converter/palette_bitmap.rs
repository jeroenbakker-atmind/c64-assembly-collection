use crate::{
    image_container::{Image, Palette4BitmapImage},
    palette::Palette4,
};

pub fn convert_to_palette4(image: &dyn Image, palette: Palette4) -> Palette4BitmapImage {
    assert!(image.width() % 4 == 0);
    assert!(image.height() % 8 == 0);

    let mut result = Palette4BitmapImage {
        width: image.width(),
        height: image.height(),
        bitmap: vec![0; image.width() / 4 * image.height()],
        palette: palette,
    };
    for y in 0..image.height() {
        for x in 0..image.width() {
            let color = image.get_pixel_color(x, y);
            let palette_index = result.palette.get_nearest_color_index(color);
            result.set_pixel_palette_index(x, y, palette_index);
        }
    }
    return result;
}
