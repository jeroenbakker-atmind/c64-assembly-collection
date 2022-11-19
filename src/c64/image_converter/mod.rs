mod standard_text;
pub use standard_text::*;

use crate::image_container::Image;

pub trait ImageConverter {
    type ResultType;

    fn convert(&self, input: &dyn Image) -> Self::ResultType;
}
