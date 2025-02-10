use std::{collections::HashSet, ops::Index, slice::Iter};

use super::{
    bit_char::{BitCharImage, BitEncodedChar},
    Image,
};

#[derive(Debug)]
pub struct ImageSequence<I>
where
    I: Image,
{
    pub images: Vec<I>,
}

impl<I> ImageSequence<I>
where
    I: Image,
{
    pub fn new() -> ImageSequence<I> {
        ImageSequence { images: Vec::default() }
    }

    pub fn push(&mut self, image: I) {
        self.images.push(image);
    }

    pub fn len(&self) -> usize {
        self.images.len()
    }

    pub fn iter(&self) -> Iter<I> {
        self.images.iter()
    }
}

impl<I: Image> Index<usize> for ImageSequence<I> {
    type Output = I;

    fn index(&self, index: usize) -> &Self::Output {
        &self.images[index]
    }
}

impl ImageSequence<BitCharImage> {
    pub fn all_unique_chars(&self) -> HashSet<BitEncodedChar> {
        let mut unique = HashSet::<u64>::new();
        for image in &self.images {
            unique.extend(image.all_unique_chars());
        }
        unique
    }

    /// Get all offsets that doesn't alter its content during the whole animation.
    /// Result is ordered from smallest offset to largest offset.
    pub fn all_static_offsets_and_chars(&self) -> (Vec<usize>, HashSet<u64>) {
        let mut result = vec![];
        let mut result_chars = HashSet::new();
        for (offset, char) in self.images[0].chars.iter().enumerate() {
            let mut is_static = true;
            for image in &self.images {
                if image.chars[offset] != *char {
                    is_static = false;
                    break;
                }
            }

            if is_static {
                result_chars.insert(*char);
                result.push(offset)
            }
        }

        (result, result_chars)
    }
}
