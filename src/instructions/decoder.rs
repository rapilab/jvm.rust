use crate::instructions::instruction::{get_instruction, NoOperandsInstruction, NullOperandsInstruction};
use crate::instructions::exec::InstructionExec;
use crate::classfile::class_file_stream::ClassFileStream;

pub fn decoder(code: Vec<u8>) -> Vec<Box<dyn InstructionExec>> {
    let mut vec: Vec<Box<dyn InstructionExec>> = Vec::with_capacity(code.len());
    let mut reader = ClassFileStream::new(code.clone());

    for _i in 0..code.len() {
        vec.push(Box::new(NullOperandsInstruction {}));
    }

    while reader.current.clone() < code.len() {
        let current = reader.current.clone();
        vec[current] = decode_instruction(&mut reader);
    }

    vec
}

fn decode_instruction(reader: &mut ClassFileStream) -> Box<dyn InstructionExec> {
    let ins = reader.get_u1();
    let mut instruction = get_instruction(ins);
    instruction.fetch_operands(reader);

    instruction
}