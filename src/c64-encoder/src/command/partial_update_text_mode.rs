use crate::encoder::{writer::Writer, Encoder};

#[derive(Debug, Clone)]
pub struct PartialUpdateTextModeScreen {
    pub changes: Vec<UpdateSingleChar>,
}

impl PartialUpdateTextModeScreen {
    pub fn transition(from_bytes: &[u8], to_bytes: &[u8]) -> PartialUpdateTextModeScreen {
        let changes = from_bytes
            .iter()
            .zip(to_bytes.iter())
            .enumerate()
            .filter(|(_index, (from, to))| from != to)
            .map(|(index, (_from, to))| UpdateSingleChar {
                offset: index as u16,
                char: *to,
            })
            .collect::<Vec<UpdateSingleChar>>();
        PartialUpdateTextModeScreen { changes }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct UpdateSingleChar {
    pub offset: u16,
    pub char: u8,
}

impl Encoder for PartialUpdateTextModeScreen {
    fn byte_size(&self) -> usize {
        self.changes.len() * (2 + 1) + 2
    }

    fn encode<'a>(&self, encoded_data: &'a mut [u8]) -> &'a mut [u8] {
        let mut encoded_data = encoded_data;
        let num_changes = self.changes.len() as u16;
        encoded_data = encoded_data.add(&num_changes);
        for change in &self.changes {
            encoded_data = encoded_data.add(change);
        }
        encoded_data
    }
}

impl Encoder for UpdateSingleChar {
    fn byte_size(&self) -> usize {
        2 + 1
    }

    fn encode<'a>(&self, encoded_data: &'a mut [u8]) -> &'a mut [u8] {
        encoded_data.add(&self.offset).add(&self.char)
    }
}
