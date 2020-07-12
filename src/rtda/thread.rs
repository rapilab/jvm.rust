use crate::rtda::j_stack::JStack;
use crate::rtda::frame::Frame;

pub struct JThread {
    stack: Box<JStack>
}

impl JThread {
    pub fn new() -> JThread {
        JThread {
            stack: Box::from(JStack::new(0))
        }
    }

    pub fn push_frame(&mut self, frame: Frame) {
        self.stack.push(frame)
    }
}