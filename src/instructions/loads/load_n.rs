use crate::instructions::exec::InstructionExec;

pub struct LoadN {
    n: usize,
    d: bool,
}

impl LoadN {
    pub fn new(n: usize, d: bool) -> LoadN {
        LoadN {
            n,
            d
        }
    }
}

impl InstructionExec for LoadN {
    fn execute(&self) {

    }
}