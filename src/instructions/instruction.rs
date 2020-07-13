use crate::instructions::constants::j_const::{ConstInt, ConstNull};
use crate::instructions::constants::ldc::LDC;
use crate::instructions::control::j_return::JReturn;
use crate::instructions::exec::InstructionExec;
use crate::instructions::loads::load_n::LoadN;
use crate::instructions::opcode;
use crate::instructions::refs::get_static::GetStatic;
use crate::instructions::refs::invoke_special::InvokeSpecial;
use crate::instructions::refs::invoke_virtual::InvokeVirtual;
use crate::rtda::frame::Frame;
use crate::classfile::class_file_stream::ClassFileStream;

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

pub struct NullOperandsInstruction {}
impl InstructionExec for NullOperandsInstruction {
    fn execute(&mut self, _frame: &Frame) {}
    fn fetch_operands(&mut self, _reader: &mut ClassFileStream) {}
}

pub struct NoOperandsInstruction {}

impl NoOperandsInstruction {
    pub fn new() -> NoOperandsInstruction {
        NoOperandsInstruction {}
    }
}

impl InstructionExec for NoOperandsInstruction {
    fn execute(&mut self, _frame: &Frame) {
        println!("NoOperandsInstruction");
    }
    fn fetch_operands(&mut self, _reader: &mut ClassFileStream) {}
}

pub fn get_instruction(ins: u8) -> Box<dyn InstructionExec> {
    match ins {
        opcode::OpNop => Box::new(NoOperandsInstruction::new()),
        opcode::OpAConstNull => Box::new(ConstNull::new()),
        opcode::OpIConstM1 => Box::new(ConstInt::new(-1)),
        opcode::OpIConst0 => Box::new(ConstInt::new(0)),
        opcode::OpIConst1 => Box::new(ConstInt::new(1)),
        opcode::OpLDC => Box::new(LDC::new()),
        opcode::OpALoad0 => Box::new(LoadN::new(0, false)),
        opcode::OpInvokeSpecial => Box::new(InvokeSpecial::new()),
        opcode::OpInvokeVirtual => Box::new(InvokeVirtual::new()),
        opcode::OpReturn => Box::new(JReturn::new()),
        opcode::OpGetStatic => Box::new(GetStatic::new()),
        _ => Box::new(NoOperandsInstruction::new()),
    }
}
