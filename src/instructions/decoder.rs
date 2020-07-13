use crate::instructions::instruction::decode_instruction;
use crate::instructions::exec::InstructionExec;
use crate::classfile::class_file_stream::ClassFileStream;

pub fn decoder(code: Vec<u8>) -> Vec<Box<dyn InstructionExec>> {
    let mut vec: Vec<Box<dyn InstructionExec>> = Vec::with_capacity(code.len());
    let mut reader = ClassFileStream::new(code.clone());

    while reader.current  < code.len() {
        let ins = reader.get_u1();
        let mut instruction = decode_instruction(ins);
        instruction.fetch_operands(&mut reader);
        vec.push(instruction)
    }

    vec
}