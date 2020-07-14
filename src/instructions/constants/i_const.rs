use crate::classfile::class_file_stream::ClassFileStream;
use crate::instructions::exec::InstructionExec;
use crate::rtda::frame::Frame;
use crate::rtda::heap::slot::{EmptySlot, IntSlot, Slot};

pub struct IConst {
    k: Slot,
}

impl InstructionExec for IConst {
    fn execute(&mut self, _frame: &mut Frame) {}

    fn fetch_operands(&mut self, _reader: &mut ClassFileStream) {}
}

pub struct ConstNull {}
impl ConstNull {
    pub fn new() -> IConst {
        IConst {
            k: EmptySlot::new(),
        }
    }
}

pub struct ConstInt {}
impl ConstInt {
    pub fn new(n: i32) -> IConst {
        IConst { k: IntSlot::new(n) }
    }
}
