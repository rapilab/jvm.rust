use byteorder::{ByteOrder, BigEndian};
use crate::oops::constant_pool::CpEntry;


#[derive(Debug, Clone)]
pub struct InstanceKlass {
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool_count: u8,
    pub constant_pool_entries: Vec<CpEntry>,
    pub klass_name: String,
    pub super_klass_name: String
}

impl InstanceKlass {
    pub fn new() -> InstanceKlass {
        InstanceKlass {
            minor_version: 0,
            major_version: 0,
            constant_pool_count: 0,
            constant_pool_entries: vec![],
            klass_name: String::from(""),
            super_klass_name: String::from("")
        }
    }

    pub fn set_minor_version(&mut self, vector: Vec<u8>) {
        self.minor_version = BigEndian::read_u16(&vector);
    }

    pub fn set_major_version(&mut self, vector: Vec<u8>) {
        self.major_version = BigEndian::read_u16(&vector);
    }

    pub fn set_class_name(&mut self, index: u16) {
        let entry = self.constant_pool_entries[index as usize].clone();
        let mut class_name: String = String::from("");
        if let CpEntry::Class { idx } = entry {
            let name = self.constant_pool_entries[idx as usize].clone();
            if let CpEntry::Utf8 { val } = name {
                class_name = val;
            }
        }
        self.klass_name = String::from(class_name);
    }
}