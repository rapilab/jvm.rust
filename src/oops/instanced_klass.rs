use byteorder::{ByteOrder, BigEndian};
use crate::oops::constant_pool::CpEntry;


#[derive(Debug, Clone)]
pub struct InstanceKlass {
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool_count: u8,
    pub constant_pool_entries: Vec<CpEntry>
}

impl InstanceKlass {
    pub fn new() -> InstanceKlass {
        InstanceKlass {
            minor_version: 0,
            major_version: 0,
            constant_pool_count: 0,
            constant_pool_entries: vec![]
        }
    }

    pub fn set_minor_version(&mut self, vector: Vec<u8>) {
        self.minor_version = BigEndian::read_u16(&vector);
    }

    pub fn set_major_version(&mut self, vector: Vec<u8>) {
        self.major_version = BigEndian::read_u16(&vector);
    }

    pub fn set_name(&mut self) {

    }
}