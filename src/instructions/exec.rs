use crate::rtda::frame::Frame;

pub trait InstructionExec {
    fn execute(&self, frame: &Frame);
}
