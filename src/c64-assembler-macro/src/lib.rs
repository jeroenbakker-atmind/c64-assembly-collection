use proc_macro::{token_stream::IntoIter, TokenStream, TokenTree};

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
            if name == "include_vic20_defines" {
                lines.push("    .add_vic20()".to_string());
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
                if let Some(TokenTree::Literal(module_name)) = iter.next() {
                    line.push(format!("{module_name})"));
                    lines.push(line.join(""));
                }
            }
            if name == "instructions" {
                let _eq = iter.next().unwrap();
                if let Some(TokenTree::Group(sub_tree)) = iter.next() {
                    lines.push("    .instructions(".to_string());
                    lines.push(build_instructions(sub_tree.stream()));
                    lines.push("    )".to_string());
                }
            }
            if name == "function" {
                let _eq = iter.next().unwrap();
                if let Some(TokenTree::Group(sub_tree)) = iter.next() {
                    lines.push("    .function(".to_string());
                    lines.push(build_function(sub_tree.stream()));
                    lines.push("    )".to_string());
                }
            }
        }
    }
    lines.push("    .finalize()".to_string());
    lines.join("\n")
}

fn build_function(input: TokenStream) -> String {
    let mut lines = Vec::<String>::default();
    lines.push("  FunctionBuilder::default()".to_string());
    let mut iter = input.into_iter();
    while let Some(tree) = iter.next() {
        if let TokenTree::Ident(identifier) = tree {
            let name = identifier.to_string();
            if name == "name" {
                let mut line = Vec::<String>::default();
                line.push("    .name(".to_string());
                let _eq = iter.next().unwrap();
                if let Some(TokenTree::Literal(function_name)) = iter.next() {
                    line.push(format!("{function_name})"));
                    lines.push(line.join(""));
                }
            }
            if name == "instructions" {
                let _eq = iter.next().unwrap();
                if let Some(TokenTree::Group(sub_tree)) = iter.next() {
                    lines.push("    .instructions(".to_string());
                    lines.push(build_instructions(sub_tree.stream()));
                    lines.push("    )".to_string());
                }
            }
        }
    }
    lines.push("    .finalize()".to_string());
    lines.join("\n")
}

fn build_address_mode(line: &mut Vec<String>, iter: &mut IntoIter) {
    match iter.next() {
        Some(TokenTree::Punct(punct)) => {
            if punct.to_string() == "#" {
                line.push("_imm".to_string());
                build_address_mode_imm(line, iter);
            }
        }
        Some(_) => todo!(),
        None => todo!(),
    }
}
fn build_address_mode_imm(line: &mut Vec<String>, iter: &mut IntoIter) {
    loop {
        match iter.next() {
            Some(TokenTree::Punct(punct)) => {
                if punct.to_string() == "$" {
                    if let Some(TokenTree::Literal(value)) = iter.next() {
                        line.push("(".to_string());
                        line.push(value.to_string());
                        line.push(")".to_string());
                        return;
                    }
                }
            }
            Some(_) => todo!(),
            None => todo!(),
        }
    }
}
fn build_instructions(input: TokenStream) -> String {
    let mut lines = Vec::<String>::default();
    lines.push("  InstructionBuilder::default()".to_string());
    let mut iter = input.into_iter();
    while let Some(tree) = iter.next() {
        if let TokenTree::Ident(identifier) = tree {
            let name = identifier.to_string();
            match name.as_str() {
                "include_basic_header" => {
                    lines.push("    .add_basic_header()".to_string());
                }
                "lda" => {
                    let mut line = Vec::default();
                    line.push(format!("    .{name}"));
                    build_address_mode(&mut line, &mut iter);
                    lines.push(line.join(""));
                }

                "brk" | "cld" | "cli" | "clv" | "dex" | "dey" | "inx" | "iny" | "nop" | "pha"
                | "psr" | "pla" | "plp" | "rti" | "sed" | "sei" | "tax" | "tay" | "tsx" | "txa"
                | "txs" | "tya" | "clc" | "rts" => {
                    lines.push(format!("    .{name}()"));
                }

                &_ => {
                    lines.push(format!("    .label(\"{name}\")"));
                    iter.next().unwrap();
                }
            }
        }
    }
    lines.push("    .finalize()".to_string());
    lines.join("\n")
}

#[proc_macro]
pub fn module(input: TokenStream) -> TokenStream {
    dbg!(input.clone());
    let mut lines = Vec::<String>::default();
    lines.push("{".to_string());
    lines.push("  use c64_assembler::builder::{*};".to_string());
    lines.push(build_module(input));
    lines.push("}".to_string());
    println!("{:#?}", lines.join("\n"));
    lines.join("\n").parse().unwrap()
}

#[proc_macro]
pub fn function(input: TokenStream) -> TokenStream {
    dbg!(input.clone());
    let mut lines = Vec::<String>::default();
    lines.push("{".to_string());
    lines.push("  use c64_assembler::builder::{*};".to_string());
    lines.push(build_function(input));
    lines.push("}".to_string());
    println!("{:#?}", lines.join("\n"));
    lines.join("\n").parse().unwrap()
}

#[proc_macro]
pub fn instructions(input: TokenStream) -> TokenStream {
    dbg!(input.clone());
    let mut lines = Vec::<String>::default();
    lines.push("{".to_string());
    lines.push("  use c64_assembler::builder::{*};".to_string());
    lines.push(build_instructions(input));
    lines.push("}".to_string());
    println!("{:#?}", lines.join("\n"));
    lines.join("\n").parse().unwrap()
}
