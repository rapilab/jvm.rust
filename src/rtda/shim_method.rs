use crate::rtda::heap::member::ClassMember;
use crate::rtda::heap::j_method::{JMethod, MethodData};
use crate::rtda::frame::{Frame, LocalVars, OperandStack};
use crate::rtda::thread::Thread;

pub fn new_shim_member(name: String) -> ClassMember {
    ClassMember {
        name,
        descriptor: "".to_string(),
        signature: "".to_string()
    }
}

pub fn shim_return_method() -> JMethod {
    // let member = new_shim_member(String::from("<return>"));
    let mut data = MethodData::new();
    data.code = vec![0xb1];

    let mut method = JMethod::new();
    method.method_data = data;

    method
}

pub fn new_shim_frame(thread: &Thread) -> Frame {
    Frame {
        local_vars: LocalVars::new(),
        operand_stack: OperandStack::new(),
        thread,
        method: JMethod::new(),
        max_locals: 0,
        max_stack: 0,
        next_pc: 0
    }
}

pub fn get_bootstrap_method() {

}