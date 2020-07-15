use crate::instructions::exec::InstructionExec;
use crate::rtda::frame::Frame;

use crate::classfile::class_file_stream::ClassFileStream;
use crate::classfile::constant_member_ref::ConstantMemberRef;
use crate::rtda::heap::instanced_klass::InstanceKlass;
use crate::rtda::heap::j_constant::{JConstant, JField};

#[derive(Clone)]
pub struct GetStatic {
    pub index: usize,
    pub field: Box<JField>,
}

impl GetStatic {
    pub fn new() -> GetStatic {
        let member_ref = ConstantMemberRef {
            class: Box::new(InstanceKlass::new()),
            class_name: "".to_string(),
            name: "".to_string(),
            descriptor: "".to_string(),
        };
        GetStatic {
            index: 0,
            field: Box::new(JField { member_ref }),
        }
    }
}

impl InstructionExec for GetStatic {
    fn execute(&mut self, frame: &mut Frame) {
        let cp = frame.clone().get_constant_pool();
        let option = cp.get(self.index);
        match option {
            None => {}
            Some(entry) => match entry {
                JConstant::ConstantField(field) => self.field = Box::from(field.clone()),
                _ => {}
            },
        }
    }

    fn fetch_operands(&mut self, reader: &mut ClassFileStream) {
        self.index = reader.read_u16() as usize;
    }
}
