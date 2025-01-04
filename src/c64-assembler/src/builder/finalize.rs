use std::collections::HashMap;

use crate::memory::user_count::UserCount;

use super::{application_builder::ApplicationBuilder, instruction_builder::InstructionBuilder};

pub fn finalize(application: &mut ApplicationBuilder) {
    defines_update_user_count(application);
}

fn defines_update_user_count(application: &mut ApplicationBuilder) {
    let mut define_users = HashMap::new();
    for define in &application.defines {
        let user_count = count_users(application, &define.name);
        if user_count > 0 {
            define_users.insert(define.name.clone(), user_count);
        }
    }

    for (define_name, user_count) in define_users {
        let define = application.define_mut(&define_name);
        for _ in 0..user_count {
            define.user_increase();
        }
    }
}

fn count_users(application: &ApplicationBuilder, name: &String) -> usize {
    let mut result = 0;
    for module in &application.modules {
        result += count_users_instructions(&module.instructions, name)
    }
    result
}

fn count_users_instructions(instructions: &InstructionBuilder, name: &String) -> usize {
    let mut result = 0;
    for instruction in &instructions.instructions {
        match &instruction.address_mode {
            crate::memory::address_mode::AddressMode::Absolute(address_reference)
            | crate::memory::address_mode::AddressMode::AbsoluteX(address_reference)
            | crate::memory::address_mode::AddressMode::AbsoluteY(address_reference)
            | crate::memory::address_mode::AddressMode::IndexedIndirect(address_reference)
            | crate::memory::address_mode::AddressMode::IndirectIndexed(address_reference) => {
                if &address_reference.name == name {
                    result += 1;
                }
            }

            crate::memory::address_mode::AddressMode::Implied
            | crate::memory::address_mode::AddressMode::Immediate(_) => {}
        }
    }
    result
}
