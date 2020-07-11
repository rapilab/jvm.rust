use std::fs::File;
use std::io::Read;
use std::thread::current;

#[derive(Debug, Clone)]
pub struct ClassFileStream {
    source: Vec<u8>,
    current: u8,
}

impl ClassFileStream {
    pub fn new(buffer: Vec<u8>) -> ClassFileStream {
        ClassFileStream {
            source: buffer,
            current: 0,
        }
    }

    pub fn get_u2(&mut self) {
        // self.source[self.current..self.current + 2]
    }

    pub fn get_u4(&mut self) {
        // let res = self.source[self.current..self.current + 4];
        // self.current += 4;
        // res
    }
}