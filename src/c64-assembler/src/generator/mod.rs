use crate::builder::application_builder::ApplicationBuilder;

pub mod dasm;

pub trait Generator {
    type Output;
    fn generate(self, application: ApplicationBuilder) -> Self::Output;
}
