//! Update screen chars using a run-length encoding.
//!
//! There are 4 RLE command types:
//! - Set the next n chars to one specific value
//! - Set the next n chars to the next values. For each char that will be updated will contain a byte
//! - Skip the next n chars.
//! - Auto increment the next n chars. Will be followed by a single byte, but each next byte will be incremented
//!

use crate::encoder::{writer::Writer, Encoder};

#[derive(Debug, Clone)]
pub enum RLECommand {
    UpdateWithSingleValue(u8),
    UpdateValues(Vec<u8>),
    SkipValues,
    AutoIncrement(u8),
}

pub const RLE_MASK_UPDATE_WITH_SINGLE_VALUE: u8 = 0b00000000;
pub const RLE_MASK_UPDATE_VALUES: u8 = 0b01000000;
pub const RLE_MASK_SKIP_VALUES: u8 = 0b10000000;
pub const RLE_MASK_AUTO_INCREMENT: u8 = 0b11000000;
pub const RLE_MASK_BITS: u8 = 0b11000000;

#[derive(Debug, Clone)]
pub struct RLEPacket {
    // Number of chars this package covers. May not go over 64.
    pub num_screen_chars: u8,
    pub command: RLECommand,
}

#[derive(Debug, Clone, Default)]
pub struct UpdateScreenCharsRLE {
    pub rle_packets: Vec<RLEPacket>,
}

impl UpdateScreenCharsRLE {
    pub fn transition(from_screen_chars: &[u8], to_screen_chars: &[u8]) -> UpdateScreenCharsRLE {
        let mut result = UpdateScreenCharsRLE::default();

        let mut commands_per_screen_char = vec![RLE_MASK_UPDATE_VALUES; to_screen_chars.len()];
        for offset in 0..to_screen_chars.len() {
            if from_screen_chars[offset] == to_screen_chars[offset] {
                commands_per_screen_char[offset] = RLE_MASK_SKIP_VALUES;
            }
        }

        for offset in 0..to_screen_chars.len() {
            if commands_per_screen_char[offset] != RLE_MASK_UPDATE_VALUES {
                continue;
            }
            if offset < to_screen_chars.len() - 1
                && commands_per_screen_char[offset + 1] == RLE_MASK_UPDATE_VALUES
                && to_screen_chars[offset] == to_screen_chars[offset + 1]
            {
                commands_per_screen_char[offset] = RLE_MASK_UPDATE_WITH_SINGLE_VALUE;
            }
            if offset != 0
                && commands_per_screen_char[offset - 1] == RLE_MASK_UPDATE_WITH_SINGLE_VALUE
                && to_screen_chars[offset] == to_screen_chars[offset - 1]
            {
                commands_per_screen_char[offset] = RLE_MASK_UPDATE_WITH_SINGLE_VALUE;
            }
        }

        for offset in 0..to_screen_chars.len() {
            if commands_per_screen_char[offset] != RLE_MASK_UPDATE_VALUES {
                continue;
            }
            if offset < to_screen_chars.len() - 1
                && to_screen_chars[offset + 1] > 0
                && to_screen_chars[offset] == to_screen_chars[offset + 1] - 1
            {
                println!("{}, {}", to_screen_chars[offset], to_screen_chars[offset + 1]);
                commands_per_screen_char[offset] = RLE_MASK_AUTO_INCREMENT;
            }
            if offset != 0
                && to_screen_chars[offset - 1] != 255
                && to_screen_chars[offset] == to_screen_chars[offset - 1] + 1
            {
                println!("{}, {}", to_screen_chars[offset - 1], to_screen_chars[offset]);
                commands_per_screen_char[offset] = RLE_MASK_AUTO_INCREMENT;
            }
        }

        // Remove trailing SKIPS.
        while !commands_per_screen_char.is_empty() && *commands_per_screen_char.last().unwrap() == RLE_MASK_SKIP_VALUES
        {
            commands_per_screen_char.pop();
        }

        let mut offset = 0;
        while offset < commands_per_screen_char.len() {
            let rle_command_mask = commands_per_screen_char[offset];
            let mut num_screen_chars = commands_per_screen_char
                .iter()
                .skip(offset)
                .take_while(|mask| **mask == rle_command_mask)
                .count();

            while num_screen_chars != 0 {
                let num_screen_chars_in_packet = if num_screen_chars >= 63 { 63 } else { num_screen_chars } as u8;
                match rle_command_mask {
                    RLE_MASK_SKIP_VALUES => {
                        result.rle_packets.push(RLEPacket {
                            num_screen_chars: num_screen_chars_in_packet,
                            command: RLECommand::SkipValues,
                        });
                    }
                    RLE_MASK_UPDATE_WITH_SINGLE_VALUE => {
                        let mut screen_char = to_screen_chars[offset];
                        let mut packets = vec![RLEPacket {
                            num_screen_chars: 0,
                            command: RLECommand::UpdateWithSingleValue(screen_char),
                        }];
                        for add_offset in 0..num_screen_chars_in_packet {
                            let screen_char_at_offset = to_screen_chars[offset + add_offset as usize];
                            if screen_char_at_offset != screen_char {
                                packets.push(RLEPacket {
                                    num_screen_chars: 0,
                                    command: RLECommand::UpdateWithSingleValue(screen_char_at_offset),
                                });
                                screen_char = screen_char_at_offset;
                            }

                            let packet = packets.last_mut().unwrap();
                            packet.num_screen_chars += 1;
                        }

                        result.rle_packets.append(&mut packets);
                    }
                    RLE_MASK_AUTO_INCREMENT => {
                        let mut screen_char = to_screen_chars[offset];
                        let mut packets = vec![RLEPacket {
                            num_screen_chars: 0,
                            command: RLECommand::AutoIncrement(screen_char),
                        }];
                        for add_offset in 0..num_screen_chars_in_packet {
                            let screen_char_at_offset = to_screen_chars[offset + add_offset as usize];
                            if screen_char_at_offset != screen_char + add_offset {
                                packets.push(RLEPacket {
                                    num_screen_chars: 0,
                                    command: RLECommand::AutoIncrement(screen_char_at_offset),
                                });
                                screen_char = screen_char_at_offset;
                            }

                            let packet = packets.last_mut().unwrap();
                            packet.num_screen_chars += 1;
                        }

                        result.rle_packets.append(&mut packets);
                    }
                    RLE_MASK_UPDATE_VALUES => {
                        let mut screen_chars = Vec::new();
                        screen_chars.extend(&to_screen_chars[offset..(offset + num_screen_chars_in_packet as usize)]);
                        result.rle_packets.push(RLEPacket {
                            num_screen_chars: num_screen_chars_in_packet,
                            command: RLECommand::UpdateValues(Vec::from(screen_chars)),
                        });
                    }
                    _ => panic!(),
                }

                num_screen_chars -= num_screen_chars_in_packet as usize;
                offset += num_screen_chars_in_packet as usize;
            }
        }
        debug_assert!(validate(&commands_per_screen_char, &result));
        debug_assert!(validate_decode(from_screen_chars, to_screen_chars, &result.rle_packets));

        result
    }
}

fn validate(expected: &[u8], source: &UpdateScreenCharsRLE) -> bool {
    let mut decoded = vec![];
    for packet in &source.rle_packets {
        let mask = match &packet.command {
            RLECommand::UpdateWithSingleValue(_) => RLE_MASK_UPDATE_WITH_SINGLE_VALUE,
            RLECommand::UpdateValues(_items) => RLE_MASK_UPDATE_VALUES,
            RLECommand::SkipValues => RLE_MASK_SKIP_VALUES,
            RLECommand::AutoIncrement(_) => RLE_MASK_AUTO_INCREMENT,
        };
        decoded.extend(vec![mask; packet.num_screen_chars as usize]);
    }
    assert_eq!(expected, decoded);
    true
}

fn validate_decode(from_screen: &[u8], to_screen: &[u8], packets: &[RLEPacket]) -> bool {
    let mut decoded = Vec::from(from_screen);
    let mut packet_per_offset = Vec::<RLEPacket>::new();
    for packet in packets {
        for _char in 0..packet.num_screen_chars {
            packet_per_offset.push(packet.clone());
        }
    }

    let mut offset = 0;
    for packet in packets {
        println!("- {}: {:?}", offset, packet);
        offset += packet.num_screen_chars as usize;
    }

    let mut offset = 0;
    for packet in packets {
        match &packet.command {
            RLECommand::UpdateWithSingleValue(value) => {
                for _add_offset in 0..packet.num_screen_chars {
                    decoded[offset] = *value;
                    offset += 1;
                }
            }
            RLECommand::UpdateValues(items) => {
                assert_eq!(packet.num_screen_chars as usize, items.len());
                for item in items {
                    decoded[offset] = *item;
                    offset += 1;
                }
            }
            RLECommand::SkipValues => offset += packet.num_screen_chars as usize,
            RLECommand::AutoIncrement(start_value) => {
                for add_offset in 0..packet.num_screen_chars {
                    decoded[offset] = start_value + add_offset;
                    offset += 1;
                }
            }
        }
    }

    for (offset, (expected, decoded)) in to_screen.iter().zip(decoded.iter()).enumerate() {
        assert_eq!(
            expected, decoded,
            "Char at {offset} are not equal, {:?}",
            packet_per_offset[offset]
        );
    }

    assert_eq!(to_screen, decoded);

    true
}

impl Encoder for RLEPacket {
    fn byte_size(&self) -> usize {
        let result = match &self.command {
            RLECommand::UpdateWithSingleValue(_) => 1,
            RLECommand::UpdateValues(items) => items.len(),
            RLECommand::SkipValues => 0,
            RLECommand::AutoIncrement(_) => 1,
        } + 1;
        result
    }

    fn encode<'a>(&self, encoded_data: &'a mut [u8]) -> &'a mut [u8] {
        assert!(self.num_screen_chars < 64);
        let mut encoded_data = encoded_data;
        match &self.command {
            RLECommand::UpdateWithSingleValue(value) => {
                let encoded_command = self.num_screen_chars | RLE_MASK_UPDATE_WITH_SINGLE_VALUE;
                encoded_data = encoded_data.add(&encoded_command).add(value);
            }
            RLECommand::UpdateValues(values) => {
                let encoded_command = self.num_screen_chars | RLE_MASK_UPDATE_VALUES;
                encoded_data = encoded_data.add(&encoded_command);
                for value in values {
                    encoded_data = encoded_data.add(value);
                }
            }
            RLECommand::SkipValues => {
                let encoded_command = self.num_screen_chars | RLE_MASK_SKIP_VALUES;
                encoded_data = encoded_data.add(&encoded_command);
            }
            RLECommand::AutoIncrement(start_value) => {
                let encoded_command = self.num_screen_chars | RLE_MASK_AUTO_INCREMENT;
                encoded_data = encoded_data.add(&encoded_command).add(start_value);
            }
        }

        encoded_data
    }
}

impl Encoder for UpdateScreenCharsRLE {
    fn byte_size(&self) -> usize {
        self.rle_packets.iter().map(|rle| rle.byte_size()).sum::<usize>() + 1
    }

    fn encode<'a>(&self, encoded_data: &'a mut [u8]) -> &'a mut [u8] {
        assert!(self.rle_packets.len() < 256);
        let mut encoded_data = encoded_data;
        let num_rle_packets = self.rle_packets.len() as u8;
        encoded_data = encoded_data.add(&num_rle_packets);
        for rle_packet in &self.rle_packets {
            encoded_data = encoded_data.add(rle_packet);
        }
        encoded_data
    }
}
