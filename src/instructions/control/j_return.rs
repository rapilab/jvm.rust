use crate::instructions::exec::InstructionExec;
use crate::rtda::frame::Frame;

pub struct JReturn {

}

impl JReturn {
    pub fn new() -> JReturn {
        JReturn {

        }
    }
}

impl InstructionExec for JReturn {
    fn execute(&self, frame: &Frame) {

    }
}