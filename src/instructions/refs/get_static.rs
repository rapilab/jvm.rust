use crate::instructions::exec::InstructionExec;
use crate::rtda::frame::Frame;
use std::borrow::Borrow;
use crate::classfile::class_file_stream::ClassFileStream;

pub struct GetStatic {
    pub index: usize
}

impl GetStatic {
    pub fn new() -> GetStatic {
        GetStatic {
            index: 0
        }
    }
}

impl InstructionExec for GetStatic {
    fn execute(&self, frame: &Frame) {
        let cp = frame.clone().get_constant_pool();
    }

    fn fetch_operands(&mut self, reader: &mut ClassFileStream) {
        self.index = reader.read_u16() as usize;
    }
}