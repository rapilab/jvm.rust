use crate::instructions::exec::InstructionExec;

pub struct LDC {
    index: usize
}

impl LDC {
    pub fn new() -> LDC {
        LDC {
            index: 0
        }
    }
}

impl InstructionExec for LDC {
    fn execute(&self) {

    }
}