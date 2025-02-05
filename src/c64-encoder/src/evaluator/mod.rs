use state::State;

use crate::{
    charmap::encoding::decode_char,
    command::{UPDATE_CHARS_U16, UPDATE_TEXT_MODE_SCREEN},
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
                    println!("  char={char:03}, encoded={encoded_char:016b}, decoded={decoded_char:064b}");
                    state.charmap.chars[char as usize] = decoded_char;
                }
            } else if command_type == UPDATE_TEXT_MODE_SCREEN {
                println!(" command=UpdateTextModeScreen");
                println!("");
                for y in 0..25 {
                    for x in 0..40 {
                        let char = read_u8(demo_bytes, &mut current_ptr);
                        print!("{:02x}", char);
                        state.text_screen.bytes[y * 40 + x] = char;
                    }
                    println!("");
                }
            } else {
                panic!("detected an not implemented command type {command_type}");
            }
        }
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
