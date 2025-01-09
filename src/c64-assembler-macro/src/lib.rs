use proc_macro::TokenStream;

#[proc_macro]
pub fn application(input: TokenStream) -> TokenStream {
    let mut lines = Vec::<String>::default();
    lines.push("{".to_string());
    lines.push("use c64_assembler::builder::{*};".to_string());
    lines.push("ApplicationBuilder::default()".to_string());
    lines.push(".finalize()".to_string());
    lines.push("}".to_string());
    lines.join("\n").parse().unwrap()
}

#[proc_macro]
pub fn module(input: TokenStream) -> TokenStream {
    "".parse().unwrap()
}

#[proc_macro]
pub fn function(input: TokenStream) -> TokenStream {
    "".parse().unwrap()
}

fn instructions(input:TokenStream) -> TokenStream {
    "".parse().unwrap()
}