use crate::instructions::instruction::get_instruction;
use crate::instructions::exec::InstructionExec;
use crate::classfile::class_file_stream::ClassFileStream;

pub fn decoder(code: Vec<u8>) -> Vec<Box<dyn InstructionExec>> {
    let mut current = 0;
    let mut vec: Vec<Box<dyn InstructionExec>> = Vec::with_capacity(code.len());
    let mut reader = ClassFileStream::new(code.clone());

    loop {
        let ops_ins = code.get(current);
        match ops_ins {
            Some(ops) => {
                let ins = *ops;
                let mut instruction = get_instruction(ins);
                instruction.fetch_operands(&mut reader);
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