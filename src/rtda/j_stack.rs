use crate::rtda::frame::Frame;

pub struct JStack {
    max_size: usize,
    size: usize,
    top: Vec<Frame>
}