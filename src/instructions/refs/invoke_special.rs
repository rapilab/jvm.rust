use crate::instructions::exec::InstructionExec;
use crate::rtda::frame::Frame;
use crate::classfile::class_file_stream::ClassFileStream;

pub struct InvokeSpecial {
    pub index: usize,
}

impl InvokeSpecial {
    pub fn new() -> InvokeSpecial {
        InvokeSpecial { index: 0 }
    }
}

impl InstructionExec for InvokeSpecial {
    fn execute(&mut self, frame: &Frame) {
        let _cp = frame.clone().get_constant_pool();
    }

    fn fetch_operands(&mut self, _reader: &mut ClassFileStream) {
        self.index = _reader.read_u16() as usize;
    }
}
