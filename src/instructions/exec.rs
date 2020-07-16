use crate::classfile::class_file_stream::ClassFileStream;
use crate::rtda::frame::Frame;

pub trait InstructionExecClone {
    fn clone_box(&self) -> Box<dyn InstructionExec>;
}

impl<T> InstructionExecClone for T
    where
        T: 'static + InstructionExec + Clone,
{
    fn clone_box(&self) -> Box<dyn InstructionExec> {
        Box::new(self.clone())
    }
}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<dyn InstructionExec> {
    fn clone(&self) -> Box<dyn InstructionExec> {
        self.clone_box()
    }
}

pub trait InstructionExec: InstructionExecClone {
    fn execute(&mut self, frame: &mut Frame);
    fn fetch_operands(&mut self, _reader: &mut ClassFileStream);
}
