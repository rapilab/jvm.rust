use crate::instructions::exec::InstructionExec;
use crate::rtda::frame::Frame;

pub struct LoadN {
    n: usize,
    d: bool,
}

impl LoadN {
    pub fn new(n: usize, d: bool) -> LoadN {
        LoadN { n, d }
    }
}

impl InstructionExec for LoadN {
    fn execute(&mut self, _frame: &Frame) {}
}
