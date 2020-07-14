use crate::classfile::class_file_stream::ClassFileStream;
use crate::instructions::exec::InstructionExec;
use crate::rtda::frame::Frame;
use crate::rtda::heap::j_constant::{JConstant, JMethodRef};

pub struct InvokeVirtual {
    pub index: usize,
    pub method_ref: Option<JMethodRef>,
}

impl InvokeVirtual {
    pub fn new() -> InvokeVirtual {
        InvokeVirtual {
            index: 0,
            method_ref: None,
        }
    }
}

impl InstructionExec for InvokeVirtual {
    fn execute(&mut self, frame: &mut Frame) {
        let cp = frame.clone().get_constant_pool();
        let option = cp.get(self.index);
        if let Some(entry) = option {
            match entry {
                JConstant::ConstantMethodRef(method) => {
                    self.method_ref = Some(method.clone());
                    println!("descriptor -> {:?}", method.member_ref.descriptor);
                    println!("name -> {:?}", method.member_ref.name);
                }
                _ => {
                    println!("{:?}", entry);
                }
            }
        }
    }

    fn fetch_operands(&mut self, reader: &mut ClassFileStream) {
        self.index = reader.read_u16() as usize;
    }
}
