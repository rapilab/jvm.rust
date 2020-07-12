use crate::instructions::exec::InstructionExec;
use crate::rtda::frame::Frame;
use std::borrow::Borrow;

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
        let cp = frame.clone().get_constant_pool();
    }
}