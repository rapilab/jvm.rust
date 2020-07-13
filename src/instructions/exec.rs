use crate::classfile::class_file_stream::ClassFileStream;
use crate::rtda::frame::Frame;

pub trait InstructionExec {
    fn execute(&mut self, frame: &mut Frame);
    fn fetch_operands(&mut self, _reader: &mut ClassFileStream);
}
