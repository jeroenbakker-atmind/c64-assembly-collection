use proc_macro::{TokenStream, TokenTree};

#[proc_macro]
pub fn application(input: TokenStream) -> TokenStream {
    dbg!(input.clone());
    let mut lines = Vec::<String>::default();
    lines.push("{".to_string());
    lines.push("  use c64_assembler::builder::{*};".to_string());
    lines.push("  ApplicationBuilder::default()".to_string());
    let mut iter = input.into_iter();
    while let Some(tree) = iter.next() {
        if let TokenTree::Ident(identifier) = tree {
            let name = identifier.to_string();
            if name == "name" {
                let mut line = Vec::<String>::default();
                line.push("    .name(".to_string());
                let _eq = iter.next().unwrap();
                if let Some(TokenTree::Literal(application_name)) = iter.next() {
                    line.push(format!("{application_name})"));
                    lines.push(line.join(""));
                }
            }
            if name == "entry_point" {
                let mut line = Vec::<String>::default();
                line.push("    .entry_point(".to_string());
                let _eq = iter.next().unwrap();
                if let Some(TokenTree::Literal(entry_point)) = iter.next() {
                    line.push(format!("{entry_point})"));
                    lines.push(line.join(""));
                }
            }
            if name == "module" {
                let _eq = iter.next().unwrap();
                if let Some(TokenTree::Group(sub_tree)) = iter.next() {
                    lines.push("    .module(".to_string());
                    lines.push(build_module(sub_tree.stream()));
                    lines.push("    )".to_string());
                }
            }
        }
    }
    lines.push("    .finalize()".to_string());
    lines.push("}".to_string());
    println!("{:#?}", lines.join("\n"));
    lines.join("\n").parse().unwrap()
}

fn build_module(input: TokenStream) -> String {
    let mut lines = Vec::<String>::default();
    lines.push("  ModuleBuilder::default()".to_string());
    let mut iter = input.into_iter();
    while let Some(tree) = iter.next() {
        if let TokenTree::Ident(identifier) = tree {
            let name = identifier.to_string();
            if name == "name" {
                let mut line = Vec::<String>::default();
                line.push("    .name(".to_string());
                let _eq = iter.next().unwrap();
                if let Some(TokenTree::Literal(application_name)) = iter.next() {
                    line.push(format!("{application_name})"));
                    lines.push(line.join(""));
                }
            }
        }
    }
    lines.push("    .finalize()".to_string());
    lines.join("\n")
}

#[proc_macro]
pub fn module(input: TokenStream) -> TokenStream {
    "".parse().unwrap()
}

#[proc_macro]
pub fn function(input: TokenStream) -> TokenStream {
    "".parse().unwrap()
}

fn instructions(input: TokenStream) -> TokenStream {
    "".parse().unwrap()
}
