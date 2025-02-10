use std::{collections::HashMap, usize};

use c64::{
    image_container::{
        bit_char::{BitCharImage, BitEncodedChar},
        image_sequence::ImageSequence,
    },
    image_converter::{DitheredText, ImageConverter},
    image_io::{read_png::read_png, write_png::write_png},
};
use c64_encoder::{
    builder::{demo::DemoBuilder, frame::FrameBuilder},
    command::{
        partial_update_text_mode::PartialUpdateTextModeScreen,
        update_chars::{UpdateChar, UpdateCharsU16Encoded},
        update_text_mode_screen::UpdateTextModeScreen,
        Command,
    },
    encoder::Encoder,
    evaluator::{evaluate, state::TextScreen},
};

fn main() {
    /* Load images into an image list. */
    let mut images = ImageSequence::<BitCharImage>::new();
    for image_number in 1..=100 {
        let image = read_png(format!("resources/render/001/{image_number:04}.png").as_str());
        let bit_char_image = DitheredText {}.convert(&image);
        images.push(bit_char_image);
    }

    // Get all used chars */
    // Check for possible strategics
    let possible_strategies = Strategy::all_possible_strategies(&images);
    let mut solutions = vec![];
    for strategy in possible_strategies {
        solutions.push(strategy.build_frames(&images));
    }

    let mut best_solution = DemoBuilder::default();
    let mut best_solution_size = usize::MAX;
    for solution in solutions {
        let mut demo = DemoBuilder::default();
        for frame in solution {
            demo.frame(frame);
        }
        let solution_size = demo.byte_size();
        if solution_size < best_solution_size {
            best_solution = demo;
            best_solution_size = solution_size;
        }
    }

    assert!(best_solution_size != usize::MAX, "No solution found");

    let demo_bytes = best_solution.build();
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

/// Find best algorithm to transition from previous_text_mode_screen to text_mode_screen.
/// By testing the number of bytes needed for several algorithms. The smallest will be chosen.
///
/// Something similar could also be created for charmaps.
fn choose_best_text_mode_update(
    demo_frame: &mut FrameBuilder,
    from_screen: &UpdateTextModeScreen,
    to_screen: &UpdateTextModeScreen,
) {
    let mut best_commands;
    let mut best_byte_size;

    // Algorithm: Full update of the text mode screen.
    best_commands = vec![Command::UpdateTextModeScreen(to_screen.clone())];
    best_byte_size = best_commands[0].byte_size();

    // Algorithm: In case both screens are identical there is nothing to do.
    if to_screen.chars == from_screen.chars {
        best_commands = vec![];
        best_byte_size = 0;
    }

    // Algorithm: Partial update, one char at a time.
    let partial_update = Command::PartialUpdateTextModeScreen(PartialUpdateTextModeScreen::transition(
        &from_screen.chars,
        &to_screen.chars,
    ));
    let partial_update_byte_size = partial_update.byte_size();
    if partial_update_byte_size < best_byte_size {
        best_commands = vec![partial_update];
        best_byte_size = partial_update_byte_size;
    }
    // Algorithm: Use rle for subsequential updates including small gabs of 2 pixels, and one char at a time for the rest.
    // Algorithm: Check partial transition using run length encoding. Multiple bytes can be skipped, and multiple bytes can be written.
    // Algorithm: Check full update using run length encoding. with a clear.

    demo_frame.extend(&best_commands);
}

enum Strategy {
    OneCharmap,
    StaticWithDynamicCharset,
    Initial,
}

impl Strategy {
    fn all_possible_strategies(images: &ImageSequence<BitCharImage>) -> Vec<Strategy> {
        let mut result = vec![];
        let total_unique_chars = images.all_unique_chars().len();
        println!("image sequence contains {} unique characters", total_unique_chars);
        if total_unique_chars <= 256 {
            result.push(Strategy::OneCharmap);
        }

        let (static_offsets, unique_static_chars) = images.all_static_offsets_and_chars();
        let num_dynamic_offsets = images.images[0].chars.len() - static_offsets.len();
        let total_chars = unique_static_chars.len() + num_dynamic_offsets;
        println!(
            "image sequence contains {} static chars and {} dynamic offsets",
            unique_static_chars.len(),
            num_dynamic_offsets
        );
        if total_chars <= 256 {
            result.push(Strategy::StaticWithDynamicCharset);
        }

        result.push(Strategy::Initial);

        result
    }

    fn build_frames(&self, images: &ImageSequence<BitCharImage>) -> Vec<FrameBuilder> {
        match self {
            Strategy::Initial => build_frames_initial(images),
            Strategy::OneCharmap => build_frames_one_charmap(images),
            Strategy::StaticWithDynamicCharset => build_frames_static_with_dynamic_charset(images),
        }
    }
}

fn build_frames_initial(images: &ImageSequence<BitCharImage>) -> Vec<FrameBuilder> {
    let mut result = Vec::<FrameBuilder>::new();

    // Usages contains per encoded char which frames is using it. The list of frames is in sequential order.
    let mut usages = HashMap::<BitEncodedChar, Vec<usize>>::new();
    for (index, image) in images.iter().enumerate() {
        let frame = index + 1;
        let image_uniques = image.all_unique_chars();
        for bit_char in &image_uniques {
            usages.entry(*bit_char).or_default().push(frame);
        }
    }

    // TODO: split into ranges (key, from, till) and able to link multiple. When linked they must share the same charcode.
    // This will ensure we can implement looping.

    // Randomize the linked ranges and perform the algorithm. Do this multiple times and find the one using the least amount of byte size in the end.
    // Find out what the patterns are and focus on designing an algorithm based on the best pattern.

    // Other solution would be to add the chars per frame until all chars are filled. At that time we look for chars that are not used and reuse its position.

    // Order the usages based on the number of frames a char covers.
    // pop the last (most frames) and find a location where it fits
    // Always add a leading and trailing frame to guarantee repeating zones.
    // continue until the usage list is empty. fail when more then 256 locations are needed.
    let mut usage_keys = usages.keys().map(|key| *key).collect::<Vec<BitEncodedChar>>();
    usage_keys.sort_by_key(|key| usages[key].len());
    let max_frame = images.len() + 1;
    let mut inner_result = Vec::<Vec<Option<BitEncodedChar>>>::default();
    while let Some(key) = usage_keys.pop() {
        let mut location = None;
        // improvement could be to do it per range of frames instead of a collection of frames.
        // This will find gabs and reduce the number of needed chars. Although keeping the possible repeating frames working requires the first and last frames to use the same loc. Perhaps we can do this as a post-process.
        let frames = &usages[&key];
        for (loc, frame_slots) in inner_result.iter().enumerate() {
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
            inner_result.get_mut(location).unwrap()
        } else {
            inner_result.push(vec![None; max_frame]);
            inner_result.last_mut().unwrap()
        };
        for frame in frames {
            location.get_mut(frame - 1).map(|slot| *slot = Some(key));
            location.get_mut(*frame).map(|slot| *slot = Some(key));
            location.get_mut(frame + 1).map(|slot| *slot = Some(key));
        }
    }
    assert!(
        inner_result.len() <= 256,
        "More characters needed for (animation vs algorithm): needed={}",
        inner_result.len()
    );

    // dilate the results.
    // Ensures all slots are filled, and ensures the first slot is not filled.
    for location in inner_result.iter_mut() {
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
    let mut charmap_per_frame = vec![vec![0; inner_result.len()]; images.len() + 1];
    for (index, location) in inner_result.iter().enumerate() {
        for (frame_index, bit_char) in location.iter().enumerate() {
            charmap_per_frame[frame_index][index] = bit_char.unwrap_or_default();
        }
    }

    // Extract the changes in the char map per frame.
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
        let text_mode_screen: UpdateTextModeScreen = create_text_mode_screen(image, &frame_charmap);
        let previous_text_mode_screen = if frame > 1 {
            let previous_image = &images[frame - 2];
            create_text_mode_screen(previous_image, &frame_charmap)
        } else {
            // TODO: find most used char in text_mode_screen.
            demo_frame.fill_video_memory(2);
            UpdateTextModeScreen::filled(2)
        };
        choose_best_text_mode_update(&mut demo_frame, &previous_text_mode_screen, &text_mode_screen);
        result.push(demo_frame);
    }
    result
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

fn build_frames_one_charmap(images: &ImageSequence<BitCharImage>) -> Vec<FrameBuilder> {
    let mut result = vec![];
    todo!();
    result
}

fn build_frames_static_with_dynamic_charset(images: &ImageSequence<BitCharImage>) -> Vec<FrameBuilder> {
    let mut result = vec![];
    let (static_offsets, static_chars) = images.all_static_offsets_and_chars();
    // build the screen chars.
    let mut text_screen = TextScreen::default();
    let mut dynamic_screen_char = static_chars.len() as u8;
    let mut charmap = static_chars.into_iter().collect::<Vec<BitEncodedChar>>();
    charmap.sort();

    for (offset, char) in images[0].chars.iter().enumerate() {
        let screen_char = if static_offsets.contains(&offset) {
            charmap
                .iter()
                .enumerate()
                .filter(|(_screen_char, char_b)| *char_b == char)
                .map(|(screen_char, _char_b)| screen_char as u8)
                .next()
                .unwrap()
        } else {
            let screen_char = dynamic_screen_char;
            charmap.push(*char);
            dynamic_screen_char += 1;
            screen_char
        };
        text_screen.bytes[offset] = screen_char;
    }

    // TODO: determine best update command
    let text_screen_command = Command::UpdateTextModeScreen(UpdateTextModeScreen {
        chars: text_screen.bytes,
    });
    let update_charmap_command = Command::UpdateCharsU16Encoded(UpdateCharsU16Encoded {
        chars: charmap
            .iter()
            .enumerate()
            .map(|(screen_char, char)| UpdateChar {
                char: screen_char as u8,
                data: *char,
            })
            .collect::<Vec<UpdateChar>>(),
    });

    for (index, image) in images.iter().enumerate() {
        let mut frame_builder = FrameBuilder::default();
        let frame = index + 1;
        if frame == 1 {
            frame_builder.push(update_charmap_command.clone());
            frame_builder.push(text_screen_command.clone());
        } else {
            let mut char_updates = vec![];
            for (offset, char) in image.chars.iter().enumerate() {
                if static_offsets.contains(&offset) {
                    continue;
                }

                let prev_char = images[index - 1].chars[offset];
                if prev_char == *char {
                    // Char is the same as previous frame
                    continue;
                }

                char_updates.push(UpdateChar {
                    char: text_screen.bytes[offset],
                    data: *char,
                });
            }
            if !char_updates.is_empty() {
                frame_builder.push(Command::UpdateCharsU16Encoded(UpdateCharsU16Encoded {
                    chars: char_updates.clone(),
                }));
            }
        }

        result.push(frame_builder);
    }
    result
}
