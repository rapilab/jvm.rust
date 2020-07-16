use crate::classfile::class_file_stream::ClassFileStream;
use crate::instructions::exec::InstructionExec;
use crate::instructions::instruction_factory::{get_instruction, NullOperandsInstruction};

#[derive(Clone)]
pub struct Decode {
    pub ins: Box<dyn InstructionExec>,
}

pub fn decode_instruction(reader: &mut ClassFileStream) -> Decode {
    let ins = reader.read_u8();
    let mut instruction = get_instruction(ins);
    instruction.fetch_operands(reader);

    Decode { ins: instruction }
}

pub fn decoder(code: Vec<u8>) -> Vec<Decode> {
    let mut vec: Vec<Decode> = Vec::with_capacity(code.len());
    let mut reader = ClassFileStream::new(code.clone());

    for _i in 0..code.len() {
        vec.push(Decode {
            ins: Box::new(NullOperandsInstruction {}),
        });
    }

    while reader.current.clone() < code.len() {
        let current = reader.current.clone();
        let instruction = decode_instruction(&mut reader);
        vec[current] = instruction;
    }

    vec
}
