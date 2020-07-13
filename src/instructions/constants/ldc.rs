use crate::classfile::class_file_stream::ClassFileStream;
use crate::instructions::exec::InstructionExec;
use crate::rtda::frame::Frame;
use crate::rtda::heap::j_constant::JConstant;

pub struct LDC {
    index: usize,
}

impl LDC {
    pub fn new() -> LDC {
        LDC { index: 0 }
    }
}

impl InstructionExec for LDC {
    fn execute(&mut self, frame: &Frame) {
        let cp = frame.clone().get_constant_pool();
        let option = cp.get(self.index);
        if let Some(entry) = option {
            match entry {
                JConstant::ConstantField(_) => {}
                JConstant::ConstantInfo(entry) => {
                    println!("{:?}", entry);
                }
                _ => {}
            }
        }
    }

    fn fetch_operands(&mut self, _reader: &mut ClassFileStream) {
        self.index = _reader.read_u8() as usize;
    }
}
