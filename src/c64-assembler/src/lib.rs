//! # C64 Assembler
//!
//! The goal of this crate is to being able to compile C64 assembly directly from Rust.
//!
//! The reasoning behind it is that in a typical C64 development workflow the programs
//! are generated in a separate step and then being saved to a disk image. For generating
//! disk images there are already crates out there like cmb.
//!
//! However some projects require more control over the compilation stage and disk
//! construction stage. Having a C64 assembler written in rust can build a bridge and
//! allows custom disk formats and faster iterations during development.
//!
//! ## Modules and functions
//!
//! An [crate::builder::application::Application] is organized in [crate::builder::module::Module] and
//! [crate::builder::function::Function]. The idea is that
//! modules and functions can be switched between various implementations having the
//! same api. Functions can also be marked to be inlined (not implemented yet).
//!
//! ## Targets
//!
//! - **Done**:
//!   - [x] Support full 6502/6510 instruction set and address modes. See [crate::builder::instruction::InstructionBuilder]
//!   - [x] Use a building pattern to construct the instruction stream.
//!   - [x] Being able to output to dasm source.
//!   - [x] Being able to output directly to a PRG
//! - **Short term**:
//!   - [ ] Use macros for easier to read
//! - **Mid term**:
//!   - [ ] Able to swap out parts of a program using modules and functions
//!   - [ ] Being able to output to the macro source (allowing circular development).
//!
//! ## Usage
//!
//! ### Building pattern
//!
//! An application can be build using builder patterns.
//!
//! ```
//! use c64_assembler::builder::application::ApplicationBuilder;
//! use c64_assembler::builder::module::ModuleBuilder;
//! use c64_assembler::builder::instruction::InstructionBuilder;
//!
//! let application = ApplicationBuilder::default()
//!     .name("Set black border")
//!     .add_vic20()
//!     .module(
//!         ModuleBuilder::default()
//!             .name("main")
//!             .instructions(
//!                 InstructionBuilder::default()
//!                     .add_basic_header()
//!                     .label("main_entry_point")
//!                     .lda_imm(0x00)
//!                     .comment("Load black color")
//!                     .sta_addr("VIC20_BORDER_COLOR")
//!                     .rts()
//!                     .finalize(),
//!             )
//!             .finalize(),
//!     )
//!     .finalize();
//! ```
//!
//! ### Using macros (experimental)
//!
//! To reduce the boilerplating macros can be used. This is still under development.
//!
//! ```
//! use c64_assembler_macro::application;
//!
//! let application = application!(
//!     name="Set back border"
//!     include_vic20_defines
//!     module!(
//!         name="main"
//!         instructions!(
//!         include_basic_header
//!         main_entry_point:
//!             "Load black color into accumulator"
//!             lda #$00
//!             sta VIC20_BORDER_COLOR
//!             rts
//!         )
//!     )
//! );
//! ```
//!
//! ### Generating dasm source
//!
//! Using the [crate::generator::dasm::DasmGenerator] a dasm compatible assembly source
//! can be generated.
//!
//! ```
//! use c64_assembler::generator::Generator;
//! use c64_assembler::generator::dasm::DasmGenerator;
//! # use c64_assembler::builder::application::ApplicationBuilder;
//! # let application = ApplicationBuilder::default().finalize();
//!
//! let source = DasmGenerator::default().generate(application);
//! println!("{}", source);
//! ```
//!
//! Would output
//!
//! ```asm
//! ; --- Application: SET BACK BORDER ---
//! ; NOTE: This file is generated, do not modify
//!
//!   processor 6502
//!
//! VIC20_BORDER_COLOR = $D020
//!
//!   org $0800
//!
//! ; --- Module begin: MAIN ---
//!   byte $00, $0C, $08     ; New basic line
//!   ; 10 SYS 2062
//!   byte $0A, $00, $9E, $20, $32, $30, $36, $32
//!   byte $00, $00, $00     ; End basic program
//!
//! main_entry_point:
//!   lda #$00
//!   sta VIC20_BORDER_COLOR
//!   rts
//! ; --- Module end: MAIN ---
//! ```
//!
//! ### Generating .PRG byte stream
//!
//! Using the [crate::generator::program::ProgramGenerator] to generate the byte stream.
//! The byte stream includes the loading address.
//!
//! ```
//! use c64_assembler::generator::Generator;
//! use c64_assembler::generator::program::{ProgramGenerator, print_hexdump};
//! # use c64_assembler::builder::application::ApplicationBuilder;
//! # let application = ApplicationBuilder::default().finalize();
//!
//! let bytes = ProgramGenerator::default().generate(application);
//! print_hexdump(&bytes);
//! ```
//!
//! ```txt
//! 0000:  00 08 00 0C  08 0A 00 9E  20 32 30 36  32 00 00 00
//! 0010:  A9 00 8D 20  D0 60
//! ```
//!
pub mod builder;
pub mod generator;
pub mod instruction;
pub mod memory;

#[cfg(test)]
mod test;
