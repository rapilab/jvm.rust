use std::fs::File;
use std::io::Read;
use std::thread::current;

#[derive(Debug, Clone)]
pub struct ClassFileStream {
    source: Vec<u8>,
    current: usize,
}

impl ClassFileStream {
    pub fn new(buffer: Vec<u8>) -> ClassFileStream {
        ClassFileStream {
            source: buffer,
            current: 0,
        }
    }

    pub fn get_u2(&mut self) -> Vec<u8> {
        let mut x = vec![0; 2];
        x[..2].clone_from_slice(&self.source[self.current..self.current + 2]);
        x
    }

    pub fn get_u4(&mut self) -> Vec<u8> {
        let mut x = vec![0; 4];
        x[..4].clone_from_slice(&self.source[self.current..self.current + 4]);
        x
    }
}