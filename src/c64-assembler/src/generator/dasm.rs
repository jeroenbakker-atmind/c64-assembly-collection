use crate::{
    builder::{
        application::Application, function::Function, instruction::Instructions, module::Module,
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
            comment_column: 25,
        }
    }
}

impl Generator for DasmGenerator {
    type Output = String;

    fn generate(mut self, application: Application) -> Self::Output {
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
    fn module(&mut self, application: &Application, module: &Module) {
        self.line_new();
        self.line(format!(
            "; --- Module begin: {} ---",
            module.name.to_uppercase()
        ));
        self.instructions(application, &module.instructions);
        for function in &module.functions {
            self.function(application, function);
        }
        self.line(format!(
            "; --- Module end: {} ---",
            module.name.to_uppercase()
        ));
    }

    fn function(&mut self, application: &Application, function: &Function) {
        self.line_new();
        self.line(format!(
            ";  --- Function begin: {} ---",
            function.name.to_uppercase()
        ));
        self.line_new();
        for d in &function.documentation {
            self.line(format!("; {}", d));
        }
        self.line(format!("{}:", function.name));
        self.instructions(application, &function.instructions);

        self.line(format!(
            "; --- Function end: {} ---",
            function.name.to_uppercase()
        ));
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
            match &instruction.operation {
                Operation::ADC => line.push("adc".to_string()),
                Operation::AND => line.push("and".to_string()),
                Operation::ASL => line.push("asl".to_string()),
                Operation::BCC => line.push("bcc".to_string()),
                Operation::BCS => line.push("bcs".to_string()),
                Operation::LDA => line.push("lda".to_string()),
                Operation::LDY => line.push("ldy".to_string()),
                Operation::STA => line.push("sta".to_string()),
                Operation::JMP => line.push("jmp".to_string()),
                Operation::JSR => line.push("jsr".to_string()),
                Operation::SEC => line.push("sec".to_string()),
                Operation::CLC => line.push("clc".to_string()),
                Operation::RTS => line.push("rts".to_string()),

                Operation::Raw(bytes) => {
                    line.push(format!("byte ${:02X}", bytes[0]));
                    for i in 1..bytes.len() {
                        line.push(format!(", ${:02X}", bytes[i]));
                    }
                }
                Operation::Label(label) => line.push(format!("{}:", label)),
                Operation::BEQ => line.push("beq".to_string()),
                Operation::BIT => line.push("bit".to_string()),
                Operation::BMI => line.push("bmi".to_string()),
                Operation::BNE => line.push("bne".to_string()),
                Operation::BPL => line.push("bpl".to_string()),
                Operation::BRK => line.push("brk".to_string()),
                Operation::BVC => line.push("bvc".to_string()),
                Operation::BVS => line.push("bvs".to_string()),
                Operation::CLD => line.push("cld".to_string()),
                Operation::CLI => line.push("cli".to_string()),
                Operation::CLV => line.push("clv".to_string()),
                Operation::CMP => line.push("cmp".to_string()),
                Operation::CPX => line.push("cpx".to_string()),
                Operation::CPY => line.push("cpy".to_string()),
                Operation::DEC => line.push("dec".to_string()),
                Operation::DEX => line.push("dex".to_string()),
                Operation::DEY => line.push("dey".to_string()),
                Operation::EOR => line.push("eor".to_string()),
                Operation::INC => line.push("inc".to_string()),
                Operation::INX => line.push("inx".to_string()),
                Operation::INY => line.push("iny".to_string()),
                Operation::LDX => line.push("ldx".to_string()),
                Operation::LSR => line.push("lsr".to_string()),
                Operation::NOP => line.push("nop".to_string()),
                Operation::ORA => line.push("ora".to_string()),
                Operation::PHA => line.push("pha".to_string()),
                Operation::PHP => line.push("php".to_string()),
                Operation::PLA => line.push("pla".to_string()),
                Operation::PLP => line.push("plp".to_string()),
                Operation::ROL => line.push("rol".to_string()),
                Operation::ROR => line.push("ror".to_string()),
                Operation::RTI => line.push("rti".to_string()),
                Operation::SBC => line.push("sbc".to_string()),
                Operation::SED => line.push("sed".to_string()),
                Operation::SEI => line.push("sei".to_string()),
                Operation::STX => line.push("stx".to_string()),
                Operation::STY => line.push("sty".to_string()),
                Operation::TAX => line.push("tax".to_string()),
                Operation::TAY => line.push("tay".to_string()),
                Operation::TSX => line.push("tsx".to_string()),
                Operation::TXA => line.push("txa".to_string()),
                Operation::TXS => line.push("txs".to_string()),
                Operation::TYA => line.push("tya".to_string()),
            }

            match &instruction.address_mode {
                AddressMode::Implied => {}
                AddressMode::Immediate(immediate) => match immediate {
                    Immediate::Byte(byte) => line.push(format!(" #${byte:02X}")),
                    Immediate::Low(address_reference) => {
                        line.push(format!(" #<{}", address_reference.name))
                    }
                    Immediate::High(address_reference) => {
                        line.push(format!(" #>{}", address_reference.name))
                    }
                },
                AddressMode::Absolute(address_reference)
                | AddressMode::AbsoluteX(address_reference)
                | AddressMode::AbsoluteY(address_reference) => {
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
                AddressMode::Relative(address_reference) => todo!(),
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
