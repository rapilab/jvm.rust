use crate::instructions::exec::InstructionExec;
use crate::rtda::frame::Frame;

use crate::classfile::class_file_stream::ClassFileStream;
use crate::oops::constant_pool::CpEntry;

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
        let option = cp.get(self.index);
        match option {
            None => {},
            Some(entry) => {
                match entry {
                    CpEntry::FieldRef ( field_ref ) => {
                        // println!("clz_idx {:?}, nt_dx {:}", clz_idx, nt_idx)

                    },
                    _ => {}
                }
            },
        }
    }

    fn fetch_operands(&mut self, reader: &mut ClassFileStream) {
        self.index = reader.read_u16() as usize;
    }
}