use std::{collections::HashMap, usize};

use c64::{
    image_container::{
        bit_char::{BitCharImage, BitEncodedChar},
        image_sequence::ImageSequence,
    },
    image_converter::{DitheredText, ImageConverter},
    image_io::{read_png::read_png, write_png::write_png},
};
use c64_colors::colors::Color;
use c64_encoder::{
    builder::{demo::DemoBuilder, frame::FrameBuilder},
    command::{
        clear_screen_chars::ClearScreenChars,
        partial_update_text_mode::PartialUpdateTextModeScreen,
        set_palette4::SetPalette4,
        update_chars::{UpdateChar, UpdateCharsU16Encoded},
        update_chars_ranged::{UpdateCharRanged, UpdateCharsRangedU16Encoded},
        update_screen_chars_rle::UpdateScreenCharsRLE,
        update_text_mode_screen::UpdateTextModeScreen,
        Command,
    },
    encoder::{utils::print_vechex, Encoder},
    evaluator::{evaluate, state::TextScreen},
};

fn main() {
    encode_act(1, 100);
    encode_act(2, 1);
}

fn encode_act(act: u32, number_of_frames: usize) {
    /* Load images into an image list. */
    let mut images = ImageSequence::<BitCharImage>::new();
    for image_number in 1..=number_of_frames {
        let image = read_png(format!("resources/render/{act:03}/{image_number:04}.png").as_str());
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

    let frame_states = evaluate(&demo_bytes);
    for (index, frame_state) in frame_states.iter().enumerate() {
        write_png(
            format!("resources/render/debug.{act:03}.{:04}.png", index).as_str(),
            frame_state,
        );
    }

    print_vechex(&demo_bytes);
    println!("size in bytes: {:?}", demo_bytes.len());
}

/// Find best algorithm to transition from previous_text_mode_screen to text_mode_screen.
/// By testing the number of bytes needed for several algorithms. The smallest will be chosen.
///
/// Something similar could also be created for charmaps.
fn choose_best_text_mode_update(
    from_screen: &[u8; 1000],
    to_screen: &[u8; 1000],
    allow_partial_updates: bool,
    allow_clear_and_update: bool,
) -> (Vec<Command>, usize) {
    let mut best_commands;
    let mut best_byte_size;

    // Algorithm: Full update of the text mode screen.
    best_commands = vec![Command::UpdateTextModeScreen(UpdateTextModeScreen {
        chars: to_screen.clone(),
    })];
    best_byte_size = byte_size(&best_commands);

    if allow_clear_and_update {
        let mut screen_char_count = HashMap::<u8, usize>::new();
        for screen_char in to_screen {
            *(screen_char_count.entry(*screen_char).or_default()) += 1;
        }
        let max_occurence = screen_char_count.iter().max_by_key(|(_k, v)| **v).unwrap();
        let clear_screen = [*max_occurence.0; 1000];
        let (mut clear_and_update, _clear_byte_size) =
            choose_best_text_mode_update(&clear_screen, to_screen, true, false);

        clear_and_update.insert(
            0,
            Command::ClearScreenChars(ClearScreenChars {
                screen_char: *max_occurence.0,
            }),
        );
        let clear_byte_size = byte_size(&clear_and_update);

        if clear_byte_size < best_byte_size {
            best_commands = clear_and_update;
            best_byte_size = clear_byte_size;
        }
    }

    if allow_partial_updates {
        // Algorithm: In case both screens are identical there is nothing to do.
        if to_screen == from_screen {
            best_commands = vec![];
            best_byte_size = byte_size(&best_commands);
        }

        // Algorithm: Partial update, one char at a time.
        {
            let partial_update = vec![Command::PartialUpdateTextModeScreen(
                PartialUpdateTextModeScreen::transition(from_screen, to_screen),
            )];
            let partial_update_byte_size = byte_size(&partial_update);
            if partial_update_byte_size < best_byte_size {
                best_commands = partial_update;
                best_byte_size = partial_update_byte_size;
            }
        }
        // Algorithm: Use rle for subsequential updates including small gabs of 2 pixels, and one char at a time for the rest.
        {
            let update_screen_chars_rle = vec![Command::UpdateScreenCharsRLE(UpdateScreenCharsRLE::transition(
                from_screen,
                to_screen,
            ))];
            let update_screen_chars_rle_byte_size = byte_size(&update_screen_chars_rle);
            if update_screen_chars_rle_byte_size < best_byte_size {
                best_commands = update_screen_chars_rle;
                best_byte_size = update_screen_chars_rle_byte_size;
            }
        }

        // Algorithm: Check partial transition using run length encoding. Multiple bytes can be skipped, and multiple bytes can be written.
        // Algorithm: Check full update using run length encoding. with a clear.
    }
    (best_commands, best_byte_size)
}

fn choose_best_charset_update(
    from_charset: &[u64],
    to_charset: &[u64],
    allow_partial_updates: bool,
    _allow_clear_and_update: bool,
) -> (Vec<Command>, usize) {
    let mut best_commands = vec![];
    let mut best_byte_size = usize::MAX;

    // Early exit when no update is needed.
    if allow_partial_updates && from_charset == to_charset {
        return (vec![], 0);
    }

    // Full range update
    {
        let update_chars_ranged = vec![Command::UpdateCharsRangedU16Encoded(UpdateCharsRangedU16Encoded {
            offset: 0,
            chars: to_charset
                .iter()
                .map(|char| UpdateCharRanged { data: *char })
                .collect::<Vec<UpdateCharRanged>>(),
        })];
        let update_chars_ranged_byte_size = byte_size(&update_chars_ranged);

        if update_chars_ranged_byte_size < best_byte_size {
            best_commands = update_chars_ranged;
            best_byte_size = update_chars_ranged_byte_size;
        }
    }

    if allow_partial_updates {
        let update_chars = vec![Command::UpdateCharsU16Encoded(UpdateCharsU16Encoded {
            chars: to_charset
                .iter()
                .enumerate()
                .filter(|(index, to_screen_char)| from_charset[*index] != **to_screen_char)
                .map(|(screen_char, char)| UpdateChar {
                    char: screen_char as u8,
                    data: *char,
                })
                .collect::<Vec<UpdateChar>>(),
        })];
        let update_chars_byte_size = byte_size(&update_chars);
        if update_chars_byte_size < best_byte_size {
            best_commands = update_chars;
            best_byte_size = update_chars_byte_size;
        }
    }

    (best_commands, best_byte_size)
}
fn byte_size(commands: &[Command]) -> usize {
    commands.iter().map(|c| c.byte_size()).sum::<usize>()
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
    // Sort by usage count and then by character bits to make the generated data deterministic.
    usage_keys.sort_by_key(|key| (usages[key].len(), *key));
    let max_frame = images.len() + 1;
    let mut inner_result = Vec::<Vec<Option<BitEncodedChar>>>::default();
    // Per frame, the character slot that holds each glyph. This is the slot the screen must
    // reference; other slots may hold the same bits because of the dilation below.
    let mut slots_per_frame = vec![HashMap::<BitEncodedChar, u8>::new(); max_frame];
    while let Some(key) = usage_keys.pop() {
        let mut location = None;
        // improvement could be to do it per range of frames instead of a collection of frames.
        // This will find gabs and reduce the number of needed chars. Although keeping the possible repeating frames working requires the first and last frames to use the same loc. Perhaps we can do this as a post-process.
        let frames = &usages[&key];
        for (loc, frame_slots) in inner_result.iter().enumerate() {
            let mut loc_possible = true;
            for frame in frames {
                // A slot can only hold one glyph per frame.
                if matches!(frame_slots.get(*frame), Some(Some(_bit_char))) {
                    loc_possible = false;
                    break;
                }
                // After the last frame of a run the glyph is still displayed until the next
                // frame is written. Reserve that frame as well, so the slot cannot be reused
                // by another glyph while the old glyph is still visible.
                if is_usage_run_end(frames, *frame) && matches!(frame_slots.get(frame + 1), Some(Some(_bit_char))) {
                    loc_possible = false;
                    break;
                }
            }
            if loc_possible {
                location = Some(loc);
                break;
            }
        }

        let location = if let Some(location) = location {
            location
        } else {
            inner_result.push(vec![None; max_frame]);
            inner_result.len() - 1
        };
        for frame in frames {
            inner_result[location][*frame] = Some(key);
            slots_per_frame[*frame].insert(key, location as u8);
            if is_usage_run_end(frames, *frame) {
                if let Some(guard) = inner_result[location].get_mut(frame + 1) {
                    if guard.is_none() {
                        *guard = Some(key);
                    }
                }
            }
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
    for frame in 1..=images.len() {
        let from_charset = &charmap_per_frame[frame - 1];
        let to_charset = &charmap_per_frame[frame];
        let (best_charmap_commands, _best_charmap_size) =
            choose_best_charset_update(&from_charset, &to_charset, frame != 1, false);

        let mut demo_frame = FrameBuilder::default();
        if frame == 1 {
            demo_frame.push(Command::SetPalette4(SetPalette4 {
                palette: [Color::Black, Color::White, Color::DarkGrey, Color::Grey],
            }));
        }
        demo_frame.extend(&best_charmap_commands);

        let image = &images[frame - 1];
        let text_mode_screen: UpdateTextModeScreen = create_text_mode_screen_from_slots(image, &slots_per_frame[frame]);
        let previous_text_mode_screen = if frame > 1 {
            let previous_image = &images[frame - 2];
            create_text_mode_screen_from_slots(previous_image, &slots_per_frame[frame - 1])
        } else {
            // TODO: find most used char in text_mode_screen.
            UpdateTextModeScreen::filled(0xFF)
        };

        // Character updates are applied while the previous frame is still visible. A slot that
        // is on screen at that moment must not change, or the animation shows a mix of old and
        // new glyphs.
        debug_assert!(
            from_charset
                .iter()
                .zip(to_charset.iter())
                .enumerate()
                .all(|(slot, (from, to))| from == to || !previous_text_mode_screen.chars.contains(&(slot as u8))),
            "frame {frame} updates a character slot that is still visible in the previous frame"
        );

        // Do not clear the text screen: the previous frame is still visible while commands are
        // processed, so a clear would flash the whole screen before the new characters are written.
        // RLE and partial updates only write the cells that actually change.
        demo_frame.extend(
            &choose_best_text_mode_update(&previous_text_mode_screen.chars, &text_mode_screen.chars, true, false).0,
        );
        result.push(demo_frame);
    }
    result
}

/// Returns true when `frame` is the last frame of a run of consecutive frames.
fn is_usage_run_end(frames: &[usize], frame: usize) -> bool {
    !frames.contains(&(frame + 1))
}

fn create_text_mode_screen_from_slots(
    image: &BitCharImage,
    slots: &HashMap<BitEncodedChar, u8>,
) -> UpdateTextModeScreen {
    let mut text_mode_screen = UpdateTextModeScreen::default();
    for (index, char_bits) in image.chars.iter().enumerate() {
        text_mode_screen.chars[index] = *slots
            .get(char_bits)
            .unwrap_or_else(|| panic!("no character slot assigned for glyph {char_bits:016X}"))
    }
    text_mode_screen
}

fn build_frames_one_charmap(images: &ImageSequence<BitCharImage>) -> Vec<FrameBuilder> {
    let mut result = vec![];

    let chars = images.all_unique_chars();
    let mut charmap = chars.into_iter().collect::<Vec<BitEncodedChar>>();
    charmap.sort();

    let mut text_screen = TextScreen::default();
    for (index, image) in images.iter().enumerate() {
        let mut frame_builder = FrameBuilder::default();
        let frame = index + 1;
        let mut new_text_screen = TextScreen::default();
        for (screen_index, char) in image.chars.iter().enumerate() {
            let screen_char = charmap
                .iter()
                .enumerate()
                .find(|(_screen_char, char_in_map)| char == *char_in_map)
                .unwrap()
                .0 as u8;
            new_text_screen.screen_chars[screen_index] = screen_char;
        }

        if frame == 1 {
            let text_screen_commands =
                choose_best_text_mode_update(&text_screen.screen_chars, &new_text_screen.screen_chars, false, true).0;
            let update_charmap_commands = choose_best_charset_update(&charmap, &charmap, false, true).0;

            frame_builder.extend(&update_charmap_commands);
            frame_builder.extend(&text_screen_commands);
        } else {
            let text_screen_commands =
                choose_best_text_mode_update(&text_screen.screen_chars, &new_text_screen.screen_chars, true, true).0;

            frame_builder.extend(&text_screen_commands);
        }
        text_screen.screen_chars = new_text_screen.screen_chars;

        result.push(frame_builder);
    }
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
        text_screen.screen_chars[offset] = screen_char;
    }

    let text_screen_commands =
        choose_best_text_mode_update(&text_screen.screen_chars, &text_screen.screen_chars, false, true).0;
    let update_charmap_commands = choose_best_charset_update(&charmap, &charmap, false, true).0;

    for (index, image) in images.iter().enumerate() {
        let mut frame_builder = FrameBuilder::default();
        let frame = index + 1;
        if frame == 1 {
            frame_builder.extend(&update_charmap_commands);
            frame_builder.extend(&text_screen_commands);
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
                    char: text_screen.screen_chars[offset],
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
