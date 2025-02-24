use state::State;

use crate::{
    charmap::encoding::decode_char,
    command::{
        update_screen_chars_rle::{
            RLE_MASK_AUTO_INCREMENT, RLE_MASK_BITS, RLE_MASK_SKIP_VALUES, RLE_MASK_UPDATE_VALUES,
            RLE_MASK_UPDATE_WITH_SINGLE_VALUE,
        },
        CLEAR_SCREEN_CHAR, PARTIAL_UPDATE_TEXT_MODE_SCREEN, SET_PALETTE4, UPDATE_CHARS_RANGED_U16, UPDATE_CHARS_U16,
        UPDATE_SCREEN_CHARS_RLE, UPDATE_TEXT_MODE_SCREEN,
    },
};

pub mod state;

pub fn evaluate(demo_bytes: &[u8]) -> Vec<State> {
    let mut current_ptr = 0;
    println!("decoding {} demo-bytes", demo_bytes.len());
    let num_frames = read_u16(demo_bytes, &mut current_ptr);
    println!(" num_frames={num_frames}");

    let mut frame_states = vec![];
    let mut state = State::default();
    frame_states.push(state.clone());

    for frame in 1..=num_frames {
        println!("decoding frame={frame}");
        let num_commands = read_u16(demo_bytes, &mut current_ptr);
        println!(" num_commands={num_commands}");
        state.reset();
        for command_index in 1..=num_commands {
            println!("decoding command={frame}.{command_index}");
            let command_type = read_u8(demo_bytes, &mut current_ptr);
            println!(" command_type={command_type}");
            if command_type == UPDATE_CHARS_U16 {
                println!(" command=UpdateCharsU16Encoded");
                let num_chars = read_u8(demo_bytes, &mut current_ptr);
                println!(" num_chars={num_chars}");
                for _ in 0..num_chars {
                    let char = read_u8(demo_bytes, &mut current_ptr);
                    let encoded_char = read_u16(demo_bytes, &mut current_ptr);
                    let decoded_char = decode_char(encoded_char);
                    println!("  char={char:02X}, encoded={encoded_char:016b}, decoded={decoded_char:064b}");
                    state.charset.update_char(char, decoded_char);
                }
            } else if command_type == UPDATE_CHARS_RANGED_U16 {
                println!(" command=UpdateCharsRangedU16Encoded");
                let num_chars = read_u8(demo_bytes, &mut current_ptr);
                let offset = read_u8(demo_bytes, &mut current_ptr);
                println!(" offset={offset}");
                println!(" num_chars={num_chars}");
                for char_index in 0..num_chars {
                    let char = offset + char_index;
                    let encoded_char = read_u16(demo_bytes, &mut current_ptr);
                    let decoded_char = decode_char(encoded_char);
                    println!("  char={char:02X}, encoded={encoded_char:016b}, decoded={decoded_char:064b}");
                    state.charset.update_char(char, decoded_char);
                }
            } else if command_type == UPDATE_TEXT_MODE_SCREEN {
                println!(" command=UpdateTextModeScreen");
                println!("");
                for y in 0..25 {
                    for x in 0..40 {
                        let char = read_u8(demo_bytes, &mut current_ptr);
                        print!("{char:02X}");
                        state.text_screen.screen_chars[y * 40 + x] = char;
                    }
                    println!("");
                }
            } else if command_type == PARTIAL_UPDATE_TEXT_MODE_SCREEN {
                println!(" command=PartialUpdateTextModeScreen");
                let num_changes = read_u16(demo_bytes, &mut current_ptr);
                println!(" num_changes={num_changes}");
                for _ in 0..num_changes {
                    let offset = read_u16(demo_bytes, &mut current_ptr);
                    let char = read_u8(demo_bytes, &mut current_ptr);
                    println!("  offset={offset}, char={char:02X}");
                    state.text_screen.screen_chars[offset as usize] = char;
                }
            } else if command_type == UPDATE_SCREEN_CHARS_RLE {
                println!(" command=UpdateScreenCharsRLE");
                let num_packets = read_u8(demo_bytes, &mut current_ptr);
                println!(" num_packets={num_packets:03}");
                let mut offset = 0;
                for packet_number in 0..num_packets {
                    let header = read_u8(demo_bytes, &mut current_ptr);
                    let command_mask = header & RLE_MASK_BITS;
                    let num_screen_chars = header - command_mask;
                    print!("  packet={packet_number:03}, num_screen_chars={num_screen_chars:02}");
                    match command_mask {
                        RLE_MASK_UPDATE_WITH_SINGLE_VALUE => {
                            let screen_char = read_u8(demo_bytes, &mut current_ptr);
                            print!(", rle_command=RLECommand::UpdateWithSingleValue, screen_char={screen_char:02X}");
                            for _ in 0..num_screen_chars {
                                state.text_screen.screen_chars[offset] = screen_char;
                                offset += 1;
                            }
                        }
                        RLE_MASK_UPDATE_VALUES => {
                            print!(", rle_command=RLECommand::UpdateValues, screen_chars=[");
                            for _ in 0..num_screen_chars {
                                let screen_char = read_u8(demo_bytes, &mut current_ptr);
                                print!("{screen_char:02X},");

                                state.text_screen.screen_chars[offset] = screen_char;
                                offset += 1;
                            }
                            print!("]");
                        }
                        RLE_MASK_SKIP_VALUES => {
                            print!(", rle_command=RLECommand::SkipValues");
                            offset += num_screen_chars as usize;
                        }
                        RLE_MASK_AUTO_INCREMENT => {
                            let mut screen_char = read_u8(demo_bytes, &mut current_ptr);
                            print!(", rle_command=RLECommand::AutoIncrement, screen_char={screen_char:02X}");
                            for _ in 0..num_screen_chars {
                                state.text_screen.screen_chars[offset] = screen_char;
                                offset += 1;
                                screen_char += 1;
                            }
                        }
                        _ => panic!(),
                    }
                    println!();
                }
            } else if command_type == CLEAR_SCREEN_CHAR {
                println!(" command=ClearScreenChar");
                let char = read_u8(demo_bytes, &mut current_ptr);
                println!(" screen_char={char:02X}");
                state.text_screen.screen_chars = [char; 1000];
            } else if command_type == SET_PALETTE4 {
                println!(" command=SetPalette4");
                let color13 = read_u8(demo_bytes, &mut current_ptr);
                let color24 = read_u8(demo_bytes, &mut current_ptr);
                println!(" color_0={:02X}", color13 & 0x0F);
                println!(" color_1={:02X}", color24 & 0x0F);
                println!(" color_2={:02X}", (color13 & 0xF0) >> 4);
                println!(" color_3={:02X}", (color24 & 0xF0) >> 4);
            } else {
                panic!("detected an not implemented command type {command_type}");
            }
        }
        state.mark_used();
        frame_states.push(state.clone());
    }

    frame_states
}

fn read_u8(demo_bytes: &[u8], current_ptr: &mut usize) -> u8 {
    let result = demo_bytes[*current_ptr];
    *current_ptr += 1;
    result
}

fn read_u16(demo_bytes: &[u8], current_ptr: &mut usize) -> u16 {
    let result = read_u8(demo_bytes, current_ptr) as u16 | read_u8(demo_bytes, current_ptr) as u16 * 256;
    result
}
