use byteorder::{LittleEndian, ByteOrder};

#[derive(Debug, Clone)]
pub struct ClassFileStream {
    source: Vec<u8>,
    pub(crate) current: usize,
}

impl ClassFileStream {
    pub fn new(buffer: Vec<u8>) -> ClassFileStream {
        ClassFileStream {
            source: buffer,
            current: 0,
        }
    }

    pub fn get_u1(&mut self) -> u8 {
        let mut x = vec![0; 1];
        x[..1].clone_from_slice(&self.source[self.current..self.current + 1]);
        self.current += 1;
        x[0]
    }

    pub fn get_u2(&mut self) -> Vec<u8> {
        let mut x = vec![0; 2];
        x[..2].clone_from_slice(&self.source[self.current..self.current + 2]);
        self.current += 2;
        x
    }

    pub fn get_u4(&mut self) -> Vec<u8> {
        let mut x = vec![0; 4];
        x[..4].clone_from_slice(&self.source[self.current..self.current + 4]);
        self.current += 4;
        x
    }

    pub fn read_u16(&mut self) -> u16 {
        LittleEndian::read_u16(&self.get_u2())
    }

    pub fn read_to_length(&mut self, length: u16) -> Vec<u8> {
        let len = length as usize;
        let mut x = vec![0; len];
        x[..len].clone_from_slice(&self.source[self.current..self.current + len]);
        self.current += len;
        x
    }
}