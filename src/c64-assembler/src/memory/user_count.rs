pub trait UserCount {
    fn user_increase(&mut self);
    fn user_count(&self) -> usize;
    fn user_empty(&self) -> bool {
        self.user_count() == 0
    }
}
