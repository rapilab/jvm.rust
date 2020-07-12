use crate::instructions::exec::InstructionExec;
use crate::instructions::slot::{Slot, EmptySlot, IntSlot};

pub struct JConst {
    k: Slot
}

impl InstructionExec for JConst {
    fn execute(&self) {

    }
}

pub struct ConstNull {}
impl ConstNull {
    pub fn new() -> JConst {
        JConst {
            k: EmptySlot::new()
        }
    }
}

pub struct ConstInt {}
impl ConstInt {
    pub fn new(n: i32) -> JConst {
        JConst {
            k: IntSlot::new(n)
        }
    }
}