use crate::rtda::frame::Frame;
use crate::classfile::class_file_stream::ClassFileStream;

pub trait InstructionExec {
    fn execute(&mut self, frame: &Frame);
    fn fetch_operands(&mut self, _reader: &mut ClassFileStream) {

    }
}
