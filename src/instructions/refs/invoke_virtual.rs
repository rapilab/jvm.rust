use crate::instructions::exec::InstructionExec;
use crate::rtda::frame::Frame;

pub struct InvokeVirtual {
    pub index: usize
}

impl InvokeVirtual {
    pub fn new() -> InvokeVirtual {
        InvokeVirtual {
            index: 0
        }
    }
}

impl InstructionExec for InvokeVirtual {
    fn execute(&self, _frame: &Frame) {

    }
}