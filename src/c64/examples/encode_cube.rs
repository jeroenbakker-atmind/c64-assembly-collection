use std::collections::{HashMap, HashSet};

use c64::{
    image_container::bit_char::{BitCharImage, BitEncodedChar},
    image_converter::{DitheredText, ImageConverter},
    image_io::{read_png::read_png, write_png::write_png},
};
use c64_encoder::{
    builder::{demo::DemoBuilder, frame::FrameBuilder},
    command::{
        update_chars::{UpdateChar, UpdateCharsU16Encoded},
        update_text_mode_screen::UpdateTextModeScreen,
    },
    encoder::Encoder,
    evaluator::evaluate,
};

fn main() {
    /* Load images into an image list. */
    let mut images = vec![];
    for image_number in 1..=100 {
        let image = read_png(format!("resources/render/001/{image_number:04}.png").as_str());
        let bit_char_image = DitheredText {}.convert(&image);
        images.push(bit_char_image);
    }

    // Get all used chars */
    let mut unique = HashSet::<u64>::new();
    for image in images.iter() {
        unique.extend(image.chars.iter());
    }

    // Usages contains per encoded char which frames is using it. The list of frames is in sequential order.
    let mut usages = HashMap::<BitEncodedChar, Vec<usize>>::new();
    for (index, image) in images.iter().enumerate() {
        let frame = index + 1;
        let mut unique = HashSet::<BitEncodedChar>::new();
        unique.extend(image.chars.iter());
        for bit_char in unique {
            usages.entry(bit_char).or_default().push(frame);
        }
    }

    // Order the usages based on the number of frames a char covers.
    // pop the last (most frames) and find a location where it fits
    // Always add a leading and trailing frame to guarantee repeating zones.
    // continue until the usage list is empty. fail when more then 256 locations are needed.
    let mut usage_keys = usages.keys().map(|key| *key).collect::<Vec<BitEncodedChar>>();
    usage_keys.sort_by_key(|key| usages[key].len());
    let max_frame = images.len() + 1;
    let mut result = Vec::<Vec<Option<BitEncodedChar>>>::default();
    while let Some(key) = usage_keys.pop() {
        let mut location = None;
        // improvement could be to do it per range of frames instead of a collection of frames.
        // This will find gabs and reduce the number of needed chars. Although keeping the possible repeating frames working requires the first and last frames to use the same loc. Perhaps we can do this as a post-process.
        let frames = &usages[&key];
        for (loc, frame_slots) in result.iter().enumerate() {
            let mut loc_possible = true;
            for frame in frames {
                if matches!(frame_slots.get(frame - 1), Some(Some(_bit_char)))
                    || matches!(frame_slots.get(*frame), Some(Some(_bit_char)))
                    || matches!(frame_slots.get(frame + 1), Some(Some(_bit_char)))
                {
                    loc_possible = false;
                }
            }
            if loc_possible {
                location = Some(loc);
                break;
            }
        }

        let location = if let Some(location) = location {
            result.get_mut(location).unwrap()
        } else {
            result.push(vec![None; max_frame]);
            result.last_mut().unwrap()
        };
        for frame in frames {
            location.get_mut(frame - 1).map(|slot| *slot = Some(key));
            location.get_mut(*frame).map(|slot| *slot = Some(key));
            location.get_mut(frame + 1).map(|slot| *slot = Some(key));
        }
    }
    assert!(
        result.len() <= 256,
        "More characters needed for (animation vs algorithm): needed={}",
        result.len()
    );

    // dilate the results.
    // Ensures all slots are filled, and ensures the first slot is not filled.
    for location in result.iter_mut() {
        let mut current_bit_char = location.iter().find(|f| f.is_some()).unwrap().unwrap();
        for slot in location.iter_mut() {
            if let Some(new_bit_char) = slot {
                current_bit_char = *new_bit_char;
            } else {
                *slot = Some(current_bit_char);
            }
        }
        location[0] = None;
    }

    // Translate the result to a charmap per frame. Frame zero is filled with zeros to make it easier to detect changes.
    let mut charmap_per_frame = vec![vec![0; result.len()]; max_frame];
    for (index, location) in result.iter().enumerate() {
        for (frame_index, bit_char) in location.iter().enumerate() {
            charmap_per_frame[frame_index][index] = bit_char.unwrap_or_default();
        }
    }

    // Extract the changes in the char map per frame.
    let mut demo = DemoBuilder::default();

    for frame in 1..=100 {
        let current_charmap = &charmap_per_frame[frame - 1];
        let frame_charmap = &charmap_per_frame[frame];
        let mut update_charmap = UpdateCharsU16Encoded::default();
        for (char_index, (current_char, frame_char)) in current_charmap.iter().zip(frame_charmap).enumerate() {
            if current_char != frame_char || frame == 1 {
                update_charmap.chars.push(UpdateChar {
                    char: char_index as u8,
                    data: *frame_char,
                });
            }
        }

        let mut demo_frame = FrameBuilder::default();
        if !update_charmap.chars.is_empty() {
            demo_frame.update_charmap_u16(update_charmap);
        }

        let image = &images[frame - 1];
        let text_mode_screen = create_text_mode_screen(image, &frame_charmap);
        let previous_text_mode_screen = if frame > 1 {
            let previous_image = &images[frame - 2];
            create_text_mode_screen(previous_image, &frame_charmap)
        } else {
            // TODO: find most used char in text_mode_screen.
            demo_frame.fill_video_memory(2);
            UpdateTextModeScreen::filled(2)
        };
        choose_best_text_mode_update(&mut demo_frame, &previous_text_mode_screen, &text_mode_screen);
        demo.frame(demo_frame);
    }
    let demo_bytes = demo.build();
    println!("{:?}", demo_bytes);

    let frame_states = evaluate(&demo_bytes);
    for (index, frame_state) in frame_states.iter().enumerate() {
        write_png(
            format!("resources/render/debug.001.{:04}.png", index).as_str(),
            frame_state,
        );
    }
    println!("size in bytes: {:?}", demo_bytes.len());
}

fn create_text_mode_screen(image: &BitCharImage, charmap: &[u64]) -> UpdateTextModeScreen {
    let mut text_mode_screen = UpdateTextModeScreen::default();
    let image_charcodes = image
        .chars
        .iter()
        .map(|char_bits| {
            for (charcode, charmap_bits) in charmap.iter().enumerate() {
                if charmap_bits == char_bits {
                    return charcode as u8;
                }
            }
            panic!()
        })
        .collect::<Vec<u8>>();
    text_mode_screen
        .chars
        .iter_mut()
        .zip(image_charcodes.iter())
        .for_each(|(out_screen, in_char)| *out_screen = *in_char);

    text_mode_screen
}

/// Find best algorithm to transition from previous_text_mode_screen to text_mode_screen.
/// By testing the number of bytes needed for several algorithms. The smallest will be chosen.
fn choose_best_text_mode_update(
    demo_frame: &mut FrameBuilder,
    from_screen: &UpdateTextModeScreen,
    to_screen: &UpdateTextModeScreen,
) {
    // Early exit, when both screens are identical we don't need to do anything.
    if to_screen.chars == from_screen.chars {
        return;
    }
    // Fallback to update full screen. Hopefully we never select this one.
    let _full_update_byte_len = to_screen.byte_size();

    demo_frame.update_text_mode_screen(to_screen.clone());
}
