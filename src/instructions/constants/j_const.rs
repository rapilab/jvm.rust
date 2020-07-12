use crate::instructions::exec::InstructionExec;
use crate::instructions::slot::{Slot, EmptySlot};

pub struct JConst {
    k: Slot
}

impl InstructionExec for JConst {
    fn execute() {
        unimplemented!()
    }
}

pub struct ConstNull {}

impl ConstNull {
    pub fn new() -> ConstNull {
        let j_const = JConst {
            k: EmptySlot::new()
        };

        ConstNull {}
    }
}

impl InstructionExec for ConstNull {
    fn execute() {}
}