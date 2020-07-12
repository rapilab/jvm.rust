use crate::instructions::exec::InstructionExec;

pub struct JReturn {

}

impl JReturn {
    pub fn new() -> JReturn {
        JReturn {

        }
    }
}

impl InstructionExec for JReturn {
    fn execute(&self) {

    }
}