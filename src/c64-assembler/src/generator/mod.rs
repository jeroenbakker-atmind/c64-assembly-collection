use crate::builder::application::Application;

pub mod dasm;
pub mod program;

pub trait Generator {
    type Output;
    fn generate(self, application: Application) -> Self::Output;
}
