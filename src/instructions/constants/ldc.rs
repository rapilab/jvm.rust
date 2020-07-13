use crate::instructions::exec::InstructionExec;
use crate::rtda::frame::Frame;

pub struct LDC {
    index: usize,
}

impl LDC {
    pub fn new() -> LDC {
        LDC { index: 0 }
    }
}

impl InstructionExec for LDC {
    fn execute(&mut self, _frame: &Frame) {}
}
