use crate::charset::Charset;

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct Char {
    pub bytes: [u8; 8],
}

impl Char {
    fn invert_byte(byte: &mut u8) {
        *byte = 255_u8 - *byte;
    }

    pub fn invert(&mut self) {
        self.bytes.iter_mut().for_each(Char::invert_byte);
    }

    pub fn is_bit_set(self, x: usize, y: usize) -> bool {
        assert!((0..8).contains(&x));
        assert!((0..8).contains(&x));

        let byte = self.bytes[y];
        let mask = 128 >> x;
        byte & mask > 0
    }
}

impl From<&[u8]> for Char {
    fn from(src: &[u8]) -> Char {
        assert!(src.len() == 8);
        let mut result = Char::default();
        for i in 0..8 {
            result.bytes[i] = src[i];
        }
        result
    }
}

impl From<Char> for Vec<bool> {
    fn from(src: Char) -> Vec<bool> {
        let char_set = true;
        let char_notset = false;
        let mut bits = Vec::new();

        for y in 0..8 {
            let mut byte = src.bytes[y];
            let mut line = Vec::with_capacity(8);
            for _x in 0..8 {
                let a = if byte & 1 != 0 { char_set } else { char_notset };
                line.push(a);
                byte >>= 1;
            }
            line.reverse();
            for c in &line {
                bits.push(*c);
            }
        }
        bits
    }
}

#[derive(Default, Clone)]
pub struct Chars {
    chars: Vec<Char>,
}

impl Chars {
    pub fn get_char(&self, char_index: usize) -> Char {
        self.chars[char_index]
    }

    pub fn len(&self) -> usize {
        self.chars.len()
    }

    pub fn contains(&self, ch: Char) -> bool {
        self.index_of(ch).is_some()
    }

    pub fn index_of(&self, ch: Char) -> Option<usize> {
        for char_index in 0..self.len() {
            let ch2 = self.get_char(char_index);
            if ch2 == ch {
                return Some(char_index);
            }
        }
        None
    }

    pub fn add_char(&mut self, ch: Char) {
        self.chars.push(ch);
    }

    pub fn compress(&mut self, char_list: &mut Vec<usize>, max_len: usize) -> Vec<u8> {
        self.chars.resize_with(max_len, Char::default);

        char_list
            .iter()
            .map(|index| if *index < 255 { *index } else { 0 })
            .map(|index| index as u8)
            .collect()
    }
}

impl From<Chars> for Vec<u8> {
    fn from(src: Chars) -> Vec<u8> {
        let mut result = Vec::new();
        for ch in src.chars {
            for byte in ch.bytes {
                result.push(byte);
            }
        }
        result
    }
}

impl From<Charset> for Chars {
    fn from(src: Charset) -> Self {
        let mut chars = Chars::default();
        for bytes in src.charset().chunks(8) {
            chars.chars.push(Char::from(bytes));
        }
        let mut inverted_cards = chars.clone();
        for ch in &mut inverted_cards.chars {
            ch.invert()
        }
        chars.chars.append(&mut inverted_cards.chars);
        chars
    }
}
