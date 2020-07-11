use byteorder::{ByteOrder, LittleEndian};
use std::borrow::Borrow;

#[derive(Debug, Clone)]
pub struct InstanceKlass {
    pub minor_version: u16,
    pub major_version: u16
}

impl InstanceKlass {
    pub fn new() -> InstanceKlass {
        InstanceKlass {
            minor_version: 0,
            major_version: 0,
        }
    }

    pub fn set_minor_version(&mut self, vector: Vec<u8>) {
        self.minor_version = ((vector[0] as u16) << 8) | vector[1] as u16;
    }

    pub fn set_major_version(&mut self, vector: Vec<u8>) {
        // let v = LittleEndian::read_u16(&data);
        self.major_version = ((vector[0] as u16) << 8) | vector[1] as u16;
    }

    pub fn set_name(&mut self) {

    }
}