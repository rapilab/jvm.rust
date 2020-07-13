use crate::instructions::exec::InstructionExec;
use crate::rtda::frame::Frame;
use crate::classfile::class_file_stream::ClassFileStream;

pub struct JReturn {}

impl JReturn {
    pub fn new() -> JReturn {
        JReturn {}
    }
}

impl InstructionExec for JReturn {
    fn execute(&mut self, _frame: &Frame) {}
    fn fetch_operands(&mut self, _reader: &mut ClassFileStream) {}
}
