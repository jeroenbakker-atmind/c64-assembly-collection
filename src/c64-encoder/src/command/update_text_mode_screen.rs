use crate::encoder::{writer::Writer, Encoder};

#[derive(Debug, Clone)]
pub struct UpdateTextModeScreen {
    pub chars: [u8; 1000],
}

impl Default for UpdateTextModeScreen {
    fn default() -> Self {
        Self { chars: [0; 1000] }
    }
}

impl UpdateTextModeScreen {
    pub fn filled(char: u8) -> UpdateTextModeScreen {
        UpdateTextModeScreen { chars: [char; 1000] }
    }
}

impl Encoder for UpdateTextModeScreen {
    fn byte_size(&self) -> usize {
        1000
    }

    fn encode<'a>(&self, encoded_data: &'a mut [u8]) -> &'a mut [u8] {
        let mut encoded_data = encoded_data;
        for ch in &self.chars {
            encoded_data = encoded_data.add(ch);
        }
        encoded_data
    }
}
