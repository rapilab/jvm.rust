use crate::instructions::opcode;
use crate::instructions::exec::InstructionExec;
use crate::instructions::constants::j_const::{ConstNull, ConstInt};

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

// pub struct  InstructionStack {
//     stack: Vec<Box<dyn InstructionExec>>,
// }

pub struct NoOperandsInstruction {

}

impl NoOperandsInstruction {
    pub fn new() -> NoOperandsInstruction {
        NoOperandsInstruction{}
    }
}

impl InstructionExec for NoOperandsInstruction {
    fn execute(&self) {

    }
}

pub fn get_instruction(ops: &u8, ins: u8) -> Box<dyn InstructionExec> {
    match ins {
        opcode::OpNop => {
            Box::new(NoOperandsInstruction::new())
        }
        opcode::OpAConstNull => {
            Box::new(ConstNull::new())
        }
        opcode::OpIConstM1 => {
            Box::new(ConstInt::new(-1))
        }
        opcode::OpIConst0 => {
            Box::new(ConstInt::new(0))
        }
        opcode::OpIConst1 => {
            Box::new(ConstInt::new(1))
        }
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