mod standard_bitmap;
mod standard_text;
pub use standard_bitmap::*;
pub use standard_text::*;
pub mod palette_bitmap;

use crate::image_container::Image;

pub trait ImageConverter {
    type ResultType;

    fn convert(&self, input: &dyn Image) -> Self::ResultType;
}