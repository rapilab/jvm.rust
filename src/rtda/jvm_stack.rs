use crate::rtda::frame::Frame;

#[derive(Debug, Clone)]
pub struct JVMStack<'a> {
    max_size: usize,
    size: usize,
    top: Option<Frame<'a>>,
}

impl<'a> JVMStack<'a> {
    pub fn new(_max_size: usize) -> JVMStack {
        JVMStack {
            max_size: 0,
            size: 0,
            top: None,
        }
    }

    pub fn push(&mut self, _frame: &Frame) {
        // self.top = Some(frame.borrow());
    }

    pub fn top(&self) -> Option<Frame> {
        self.clone().top
    }
}
