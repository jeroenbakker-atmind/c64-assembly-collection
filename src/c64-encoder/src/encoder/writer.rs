use super::Encoder;

pub trait Writer {
    fn add<'a>(&'a mut self, encoder: &dyn Encoder) -> &'a mut Self;
}

impl Writer for [u8] {
    fn add<'a>(&'a mut self, encoder: &dyn Encoder) -> &'a mut Self {
        encoder.encode(self)
    }
}
