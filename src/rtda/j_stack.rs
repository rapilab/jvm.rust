use crate::rtda::frame::Frame;

#[derive(Debug, Clone)]
pub struct JStack {
    max_size: usize,
    size: usize,
    top: Option<Frame>,
}

impl JStack {
    pub fn new(_max_size: usize) -> JStack {
        JStack {
            max_size: 0,
            size: 0,
            top: None,
        }
    }

    pub fn push(&mut self, _frame: &Frame) {
        // self.top = Some(frame.borrow());
    }
}
