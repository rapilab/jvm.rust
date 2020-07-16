use crate::rtda::heap::j_constant::JConstant;
use crate::rtda::heap::j_method::JMethod;
use crate::rtda::heap::slot::Slot;
use crate::rtda::thread::Thread;

#[derive(Debug, Clone)]
pub struct LocalVars {
    slots: Vec<Slot>,
}

impl LocalVars {
    pub fn new() -> LocalVars {
        LocalVars { slots: vec![] }
    }
}

#[derive(Debug, Clone)]
pub struct OperandStack {
    size: usize,
    slots: Vec<Slot>,
}

impl OperandStack {
    pub fn new() -> OperandStack {
        OperandStack {
            size: 0,
            slots: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub struct Frame {
    pub local_vars: LocalVars,
    pub operand_stack: OperandStack,
    pub thread: Box<Thread>,
    pub method: JMethod,
    pub max_locals: u16,
    pub max_stack: u16,
    pub next_pc: u16  // Program Counter
}

impl Frame {
    pub fn new(thread: Box<Thread>, method: JMethod) -> Frame {
        Frame {
            max_locals: 0,
            max_stack: 0,
            thread,
            local_vars: LocalVars::new(),
            operand_stack: OperandStack::new(),
            method,
            next_pc: 0
        }
    }

    pub fn get_constant_pool(self) -> Vec<JConstant> {
        self.method.klass.constant_pool
    }
}
