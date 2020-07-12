use crate::instructions::exec::InstructionExec;
use crate::rtda::frame::Frame;

pub struct GetStatic {

}

impl GetStatic {
    pub fn new() -> GetStatic {
        GetStatic {

        }
    }
}

impl InstructionExec for GetStatic {
    fn execute(&self, frame: &Frame) {

    }
}