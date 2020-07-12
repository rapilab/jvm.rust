use crate::instructions::instruction::get_instruction;
use crate::instructions::exec::InstructionExec;

pub fn decoder(code: Vec<u8>) -> Vec<Box<dyn InstructionExec>> {
    let mut current = 0;
    let mut vec: Vec<Box<dyn InstructionExec>> = Vec::with_capacity(code.len());
    loop {
        let ops_ins = code.get(current);
        match ops_ins {
            Some(ops) => {
                let ins = *ops;
                let instruction = get_instruction(ins);
                vec.push(instruction)
            }
            _ => {
                break;
            }
        }

        current += 1;
    }

    vec
}