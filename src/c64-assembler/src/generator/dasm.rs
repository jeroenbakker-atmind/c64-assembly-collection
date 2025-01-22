use crate::{
    builder::{application::Application, function::Function, instruction::Instructions, module::Module},
    instruction::operation::Operation,
    memory::{
        address_mode::{AddressMode, Immediate},
        define::{Define, Value},
        user_count::UserCount,
        ZeroPage,
    },
};

use super::Generator;

pub struct DasmGenerator {
    output: Vec<String>,
    comment_column: usize,
}

impl Default for DasmGenerator {
    fn default() -> Self {
        DasmGenerator {
            output: Vec::<String>::default(),
            comment_column: 25,
        }
    }
}

impl Generator for DasmGenerator {
    type Output = String;

    fn generate(mut self, application: Application) -> Self::Output {
        self.line(format!("; --- Application: {} ---", application.name.to_uppercase()));
        self.line(format!("; NOTE: This file is generated, do not modify"));
        self.line_new();
        self.line(format!("  processor 6502"));
        self.line_new();

        for define in &application.defines {
            if !define.user_empty() {
                self.add_define(define);
            }
        }
        self.line_new();
        self.line(format!("  org ${:04X}", application.entry_point));

        for module in &application.modules {
            self.module(&application, module);
        }

        self.output.join("\n")
    }
}

impl DasmGenerator {
    fn module(&mut self, application: &Application, module: &Module) {
        self.line_new();
        self.line(format!("; --- Module begin: {} ---", module.name.to_uppercase()));
        self.instructions(application, &module.instructions);
        for function in &module.functions {
            self.function(application, function);
        }
        self.line(format!("; --- Module end: {} ---", module.name.to_uppercase()));
    }

    fn function(&mut self, application: &Application, function: &Function) {
        self.line_new();
        self.line(format!("; --- Function begin: {} ---", function.name.to_uppercase()));
        self.line_new();
        for d in &function.documentation {
            self.line(format!("; {}", d));
        }
        self.line(format!("{}:", function.name));
        self.instructions(application, &function.instructions);

        self.line(format!("; --- Function end: {} ---", function.name.to_uppercase()));
    }
}

impl DasmGenerator {
    fn instructions(&mut self, _application: &Application, instructions: &Instructions) {
        for instruction in &instructions.instructions {
            let mut line: Vec<String> = vec![];
            if let Operation::Label(_) = &instruction.operation {
                self.line_new();
            } else {
                line.push("  ".to_string());
            }

            // Add operation
            match &instruction.operation {
                Operation::Raw(bytes) => {
                    line.push(format!("byte ${:02X}", bytes[0]));
                    for i in 1..bytes.len() {
                        line.push(format!(", ${:02X}", bytes[i]));
                    }
                }
                Operation::Label(label) => line.push(format!("{}:", label)),
                _ => {
                    line.push(instruction.operation.definition().unwrap().instruction.to_string());
                }
            }

            // Add address mode
            match &instruction.address_mode {
                AddressMode::Implied => {}
                AddressMode::Immediate(immediate) => match immediate {
                    Immediate::Byte(byte) => line.push(format!(" #${byte:02X}")),
                    Immediate::Low(address_reference) => line.push(format!(" #<{}", address_reference.name)),
                    Immediate::High(address_reference) => line.push(format!(" #>{}", address_reference.name)),
                },
                AddressMode::Absolute(address_reference)
                | AddressMode::AbsoluteX(address_reference)
                | AddressMode::AbsoluteY(address_reference)
                | AddressMode::Relative(address_reference) => {
                    line.push(format!(" {}", address_reference.name));
                    if address_reference.offset != 0 {
                        line.push(format!("+{}", address_reference.offset));
                    }
                }
                AddressMode::IndexedIndirect(address_reference) => {
                    line.push(format!(" ({}", address_reference.name));
                    if address_reference.offset != 0 {
                        line.push(format!("+{}", address_reference.offset));
                    }
                    line.push("),x".to_string());
                }
                AddressMode::IndirectIndexed(address_reference) => {
                    line.push(format!(" ({}", address_reference.name));
                    if address_reference.offset != 0 {
                        line.push(format!("+{}", address_reference.offset));
                    }
                    line.push("),y".to_string());
                }
                AddressMode::Accumulator => line.push(" A".to_string()),
                AddressMode::Indirect(address_reference) => {
                    line.push(format!(" ({}", address_reference.name));
                    if address_reference.offset != 0 {
                        line.push(format!("+{}", address_reference.offset));
                    }
                    line.push(")".to_string());
                }
            }

            if instruction.comments.is_empty() {
                self.line(line.join(""));
            } else {
                let mut line_len = line.join("").len();
                let add_comments_before = line_len > self.comment_column
                    || if let Operation::Label(_label) = &instruction.operation {
                        true
                    } else {
                        false
                    };

                if add_comments_before {
                    for comment in &instruction.comments {
                        self.line(format!("  ; {}", comment));
                    }
                    self.line(line.join(""))
                } else {
                    for comment in &instruction.comments {
                        let inset = vec![" "; self.comment_column - line_len].join("");
                        line.push(format!("{}; {}", inset, comment));
                        self.line(line.join(""));
                        line.clear();
                        line_len = 0;
                    }
                }
            }
        }
    }
}

impl DasmGenerator {
    fn add_define(&mut self, define: &Define) {
        let mut line = vec![];
        line.push(define.name.clone());
        line.push("=".to_string());
        match &define.value {
            Value::Address(address) => line.push(format!("${:04X}", address)),
            Value::Zeropage(address) => line.push(format!("${:02X}", address.low())),
        }

        self.line(line.join(" "));
    }
}

impl DasmGenerator {
    fn line(&mut self, line: String) {
        self.output.push(line);
    }
    fn line_new(&mut self) {
        self.output.push(String::default());
    }
}
