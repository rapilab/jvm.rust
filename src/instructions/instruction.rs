use crate::instructions::opcode;

#[derive(Clone, Debug)]
pub enum Instruction {
    OpNop(),
    OpAConstNull(),
    OpIConstM1(),
    OpIConst0(),
    OpIConst1(),
    OpLDC(),
    OpALoad0(),
    OpInvokeSpecial(),
    OpReturn(),
    OpGetStatic(),
}

pub trait InstructionExec {

}

pub struct  InstructionStack {
    stack: Vec<Box<dyn InstructionExec>>,
}

pub struct NoOperandsInstruction {

}

impl NoOperandsInstruction {
    pub fn new() -> NoOperandsInstruction {
        NoOperandsInstruction{}
    }
}

impl InstructionExec for NoOperandsInstruction {

}

pub fn get_instruction(ops: &u8, ins: u8) -> Box<dyn InstructionExec> {
    match ins {
        opcode::OpNop => {
            Box::new(NoOperandsInstruction::new())
        }
        // opcode::OpAConstNull => {
        //     println!("ACONST_NULL")
        // }
        // opcode::OpIConstM1 => {
        //     println!("ICONST_M1")
        // }
        // opcode::OpIConst0 => {
        //     println!("ICONST_0")
        // }
        // opcode::OpIConst1 => {
        //     println!("ICONST_1")
        // }
        // opcode::OpLDC => {
        //     println!("LDC")
        // }
        // opcode::OpALoad0 => {
        //     println!("ALOAD_0")
        // }
        // opcode::OpInvokeSpecial => {
        //     println!("INVOKESPECIAL")
        // }
        // opcode::OpInvokeVirtual => {
        //     println!("INVOKEVIRTUAL")
        // }
        // opcode::OpReturn => {
        //     println!("RETURN")
        // }
        // opcode::OpGetStatic => {
        //     println!("GETSTATIC")
        // }
        _ => {
            Box::new(NoOperandsInstruction::new())
        }
    }
}