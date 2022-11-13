use cbm::Petscii;

const CHARSET_UPPER: &[u8; 1024] = include_bytes!("c64_us_upper.bin");
const CHARSET_LOWER: &[u8; 1024] = include_bytes!("c64_us_lower.bin");

#[derive(Copy, Clone)]
pub enum Charset {
    Lower,
    Upper,
}

impl Charset {
    pub fn charset(&self) -> &'static [u8; 1024] {
        match self {
            Charset::Upper => CHARSET_UPPER,
            Charset::Lower => CHARSET_LOWER,
        }
    }
}

pub fn petscii_to_charset(petscii_char: u8) -> (usize, bool) {
    let invert = petscii_char > 127;
    let charset_char = (petscii_char
        - match invert {
            false => 0,
            true => 128,
        }) as usize;

    (charset_char, invert)
}

pub fn print_petscii_char(charset: Charset, petscii_char: u8) {
    let (charset_char, invert) = petscii_to_charset(petscii_char);

    let offset = charset_char as usize * 8;
    let char_set = match invert {
        false => 'X',
        true => ' ',
    };
    let char_notset = match invert {
        false => ' ',
        true => 'X',
    };

    for y in 0..8 {
        let mut byte = charset.charset()[offset + y];
        let mut line = Vec::with_capacity(8);
        for _x in 0..8 {
            let a = if byte & 1 != 0 { char_set } else { char_notset };
            line.push(a);
            byte >>= 1;
        }
        line.reverse();
        for c in line {
            print!("{}", c);
        }
        println!();
    }
}

pub fn print_petscii(charset: Charset, petscii: Petscii) {
    let mut chars = Vec::new();
    for petscii_char in petscii {
        chars.push(petscii_to_charset(petscii_char));
    }

    for y in 0..8 {
        for (char_index, invert) in &chars {
            let offset = *char_index * 8;
            let char_set = match invert {
                false => 'X',
                true => ' ',
            };
            let char_notset = match invert {
                false => ' ',
                true => 'X',
            };
            let mut byte = charset.charset()[offset + y];
            let mut line = Vec::with_capacity(8);
            for _x in 0..8 {
                let a = if byte & 1 != 0 { char_set } else { char_notset };
                line.push(a);
                byte >>= 1;
            }
            line.reverse();
            for c in line {
                print!("{}", c);
            }
        }
        println!();
    }
}
