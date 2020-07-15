use crate::classfile::class_file_stream::ClassFileStream;
use crate::instructions::exec::InstructionExec;
use crate::rtda::frame::Frame;

#[derive(Clone)]
pub struct IReturn {}

impl IReturn {
    pub fn new() -> IReturn {
        IReturn {}
    }
}

impl InstructionExec for IReturn {
    fn execute(&mut self, _frame: &mut Frame) {}
    fn fetch_operands(&mut self, _reader: &mut ClassFileStream) {}
}
