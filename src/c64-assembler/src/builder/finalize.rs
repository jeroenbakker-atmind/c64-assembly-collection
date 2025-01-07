use std::collections::HashMap;

use crate::{
    instruction::operation::Operation,
    memory::{
        address_mode::{AddressMode, Immediate},
        user_count::UserCount,
        Address,
    },
};

use super::{application_builder::ApplicationBuilder, instruction_builder::InstructionBuilder};

pub fn finalize(application: &mut ApplicationBuilder) {
    defines_update_user_count(application);
    functions_update_user_count(application);
    update_label_addresses(application);
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

fn functions_update_user_count(application: &mut ApplicationBuilder) {
    let mut function_users = HashMap::new();
    for module in &application.modules {
        for function in &module.functions {
            let user_count = count_users(application, &function.name);
            if user_count > 0 {
                function_users.insert(function.name.clone(), user_count);
            }
        }
    }

    for module in &mut application.modules {
        for function in &mut module.functions {
            if let Some(user_count) = function_users.get(&function.name) {
                for _ in 0..*user_count {
                    function.user_increase();
                }
            }
        }
    }
}

fn count_users(application: &ApplicationBuilder, name: &String) -> usize {
    let mut result = 0;
    for module in &application.modules {
        result += count_users_instructions(&module.instructions, name);
        for function in &module.functions {
            result += count_users_instructions(&function.instructions, name);
        }
    }
    result
}

fn count_users_instructions(instructions: &InstructionBuilder, name: &String) -> usize {
    let mut result = 0;
    for instruction in &instructions.instructions {
        match &instruction.address_mode {
            AddressMode::Absolute(address_reference)
            | AddressMode::AbsoluteX(address_reference)
            | AddressMode::AbsoluteY(address_reference)
            | AddressMode::IndexedIndirect(address_reference)
            | AddressMode::IndirectIndexed(address_reference)
            | AddressMode::Immediate(Immediate::Low(address_reference))
            | AddressMode::Immediate(Immediate::High(address_reference)) => {
                if &address_reference.name == name {
                    result += 1;
                }
            }

            crate::memory::address_mode::AddressMode::Immediate(Immediate::Byte(_))
            | crate::memory::address_mode::AddressMode::Implied => {}
        }
    }
    result
}

fn update_label_addresses(application: &mut ApplicationBuilder) {
    let mut label_addresses = HashMap::<String, Address>::default();
    let mut function_addresses = HashMap::<String, Address>::default();
    let mut current_address=application.entry_point;

    let mut update_label_addresses_instructions =
        |instructions: &InstructionBuilder| {
            for instruction in &instructions.instructions {
                if let Operation::Label(label) = &instruction.operation {
                    label_addresses.insert(label.clone(), application.entry_point);
                }
                current_address += instruction.byte_size(application);
            }
        };

    for module in &application.modules {
        update_label_addresses_instructions( &module.instructions);
        for function in &module.functions {
            function_addresses.insert(function.name.clone(), application.entry_point);
            update_label_addresses_instructions(&function.instructions);
        }
    }

    application.address_lookup.extend(label_addresses);
    application.address_lookup.extend(function_addresses);
}
