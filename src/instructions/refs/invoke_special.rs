use crate::instructions::exec::InstructionExec;

pub struct InvokeSpecial {
    pub index: usize
}

impl InvokeSpecial {
    pub fn new() -> InvokeSpecial {
        InvokeSpecial {
            index: 0
        }
    }
}

impl InstructionExec for InvokeSpecial {
    fn execute(&self) {

    }
}