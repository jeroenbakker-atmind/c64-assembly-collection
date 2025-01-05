use crate::{
    builder::{
        application_builder::ApplicationBuilder, instruction_builder::InstructionBuilder,
        module_builder::ModuleBuilder,
    },
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
            comment_column: 60,
        }
    }
}

impl Generator for DasmGenerator {
    type Output = String;

    fn generate(mut self, application: ApplicationBuilder) -> Self::Output {
        self.line(format!(
            "; --- Application: {} ---",
            application.name.to_uppercase()
        ));
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
    fn module(&mut self, application: &ApplicationBuilder, module: &ModuleBuilder) {
        self.line_new();
        self.line(format!(
            "; --- Module begin: {} ---",
            module.name.to_uppercase()
        ));
        self.instructions(application, &module.instructions);
        self.line(format!(
            "; --- Module end: {} ---",
            module.name.to_uppercase()
        ));
    }
}

impl DasmGenerator {
    fn instructions(
        &mut self,
        application: &ApplicationBuilder,
        instructions: &InstructionBuilder,
    ) {
        for instruction in &instructions.instructions {
            let mut line = vec![];
            match (&instruction.operation, &instruction.address_mode) {
                (Operation::RTS, AddressMode::Implied) => self.line(format!("  rts")),
                (Operation::Raw(bytes), AddressMode::Implied) => {
                    line.push(format!("  byte ${:02X}", bytes[0]));
                    for i in 1..bytes.len() {
                        line.push(format!(", ${:02X}", bytes[i]));
                    }
                }
                (Operation::Label(label), AddressMode::Implied) => {
                    self.line_new();
                    line.push(format!("{}:", label));
                }

                (Operation::LDA, AddressMode::Immediate(Immediate::Byte(byte))) => {
                    line.push(format!("  lda #${:02X}", byte));
                }
                (Operation::STA, AddressMode::Absolute(address_ref)) => {
                    line.push(format!("  sta {}", address_ref.name));
                }

                (_, _) => unreachable!(),
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
