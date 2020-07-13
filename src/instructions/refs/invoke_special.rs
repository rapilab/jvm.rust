use crate::classfile::class_file_stream::ClassFileStream;
use crate::instructions::exec::InstructionExec;
use crate::rtda::frame::Frame;
use crate::rtda::heap::j_constant::JConstant;

pub struct InvokeSpecial {
    pub index: usize,
}

impl InvokeSpecial {
    pub fn new() -> InvokeSpecial {
        InvokeSpecial { index: 0 }
    }
}

impl InstructionExec for InvokeSpecial {
    fn execute(&mut self, frame: &mut Frame) {
        let cp = frame.clone().get_constant_pool();
        let option = cp.get(self.index);
        if let Some(entry) = option {
            match entry {
                JConstant::ConstantMethodRef(str) => {
                    println!("descriptor -> {:?}", str.member_ref.descriptor);
                    println!("name -> {:?}", str.member_ref.name);
                }
                _ => {
                    println!("{:?}", entry);
                }
            }
        }
    }

    fn fetch_operands(&mut self, _reader: &mut ClassFileStream) {
        self.index = _reader.read_u16() as usize;
    }
}
