use crate::classfile::class_file_stream::ClassFileStream;
use crate::oops::instanced_klass::InstanceKlass;
use std::borrow::Borrow;
use byteorder::{ByteOrder, LittleEndian, BigEndian};
use crate::oops::constant_pool::{ConstantInfo, CpEntry};

pub struct ClassFileParser {
    major_version: Vec<u8>,
    minor_version: Vec<u8>,
    constant_pool_count: u8,
    constant_pool_entries: Vec<CpEntry>,
    access_flags: Vec<u8>,
    this_class_index: u16,
    super_class_index: u16,
    itfs_len: Vec<u8>,
    java_fields_count: Vec<u8>,
}

fn to_u32(slice: &[u8]) -> u32 {
    slice.iter().fold((0, 1), |(acc, mul), &bit| (acc + (mul * (1 & bit as u32)), mul.wrapping_add(mul))).0
}

fn is_klass_magic(clz_read: Vec<u8>) -> bool {
    clz_read[0] != 0xca
        || clz_read[1] != 0xfe
        || clz_read[2] != 0xba
        || clz_read[3] != 0xbe
}

impl ClassFileParser {
    pub fn new(stream: ClassFileStream) -> ClassFileParser {
        let mut file_parser = ClassFileParser {
            major_version: vec![0; 2],
            minor_version: vec![0; 2],
            constant_pool_count: 0,
            constant_pool_entries: vec![],
            access_flags: vec![0; 2],
            this_class_index: 0,
            super_class_index: 0,
            itfs_len: vec![0; 2],
            java_fields_count: vec![0; 2],
        };
        file_parser.parse_stream(stream.clone());

        file_parser
    }

    fn parse_stream(&mut self, mut stream: ClassFileStream) {
        let magic = stream.get_u4();
        if is_klass_magic(magic) {
            panic!("Input file {} does not have correct magic number")
        }

        self.minor_version = stream.get_u2();
        self.major_version = stream.get_u2();
        self.constant_pool_count = BigEndian::read_u16(&stream.get_u2()) as u8;
        self.constant_pool_entries = self.parse_constant_pool(&mut stream, self.constant_pool_count as usize);
        self.access_flags = stream.get_u2();
        self.this_class_index = stream.read_u16();
        self.super_class_index = stream.read_u16();
    }

    fn parse_constant_pool(&mut self, stream: &mut ClassFileStream, size: usize) -> Vec<CpEntry> {
        let mut entries: Vec<CpEntry> = vec![];
        entries.push(CpEntry::Empty {});
        // The constant_pool table is indexed from 1
        for _i in 1..size {
            entries.push(ConstantInfo::from(stream));
        }
        entries
    }

    pub fn create_instance_klass(&mut self) -> InstanceKlass {
        let mut klass = InstanceKlass::new();
        self.fill_instance_klass(&mut klass);
        klass
    }

    fn fill_instance_klass(&mut self, klass: &mut InstanceKlass) {
        klass.set_minor_version(self.minor_version.clone());
        klass.set_major_version(self.major_version.clone());
        klass.constant_pool_entries = self.constant_pool_entries.clone();
        klass.constant_pool_count = self.constant_pool_count.clone();
        klass.set_class_name(self.this_class_index);
    }
}