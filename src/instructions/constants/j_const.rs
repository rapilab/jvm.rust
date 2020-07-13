use crate::classfile::class_file_stream::ClassFileStream;
use crate::instructions::exec::InstructionExec;
use crate::instructions::slot::{EmptySlot, IntSlot, Slot};
use crate::rtda::frame::Frame;

pub struct JConst {
    k: Slot,
}

impl InstructionExec for JConst {
    fn execute(&mut self, frame: &mut Frame) {}

    fn fetch_operands(&mut self, _reader: &mut ClassFileStream) {}
}

pub struct ConstNull {}
impl ConstNull {
    pub fn new() -> JConst {
        JConst {
            k: EmptySlot::new(),
        }
    }
}

pub struct ConstInt {}
impl ConstInt {
    pub fn new(n: i32) -> JConst {
        JConst { k: IntSlot::new(n) }
    }
}
