use crate::classfile::class_file_stream::ClassFileStream;
use crate::oops::instanced_klass::InstanceKlass;
use std::borrow::Borrow;
use byteorder::{ByteOrder, LittleEndian};

pub struct ClassFileParser {
    major_version: Vec<u8>,
    minor_version: Vec<u8>,
    this_class_index: Vec<u8>,
    super_class_index: Vec<u8>,
    itfs_len: Vec<u8>,
    java_fields_count: Vec<u8>

}

fn to_u32(slice: &[u8]) -> u32 {
    slice.iter().fold((0,1),|(acc,mul),&bit|(acc+(mul*(1&bit as u32)),mul.wrapping_add(mul))).0
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
            this_class_index: vec![0; 2],
            super_class_index: vec![0; 2],
            itfs_len: vec![0; 2],
            java_fields_count: vec![0; 2]
        };
        file_parser.parse_stream(stream.clone());

        file_parser
    }

    fn parse_stream(&mut self, mut stream: ClassFileStream) {
        let magic = stream.get_u4();
        let i = LittleEndian::read_u16(magic.borrow());
        if is_klass_magic(magic) {
            panic!("Input file {} does not have correct magic number")
        }

        self.minor_version = stream.get_u2();
        self.major_version = stream.get_u2();
    }

    pub fn create_instance_klass(&mut self) -> InstanceKlass {
        let mut klass = InstanceKlass::new();
        self.fill_instance_klass(&mut klass);
        klass
    }

    fn fill_instance_klass(&mut self, klass: &mut InstanceKlass) {
        klass.set_minor_version(self.minor_version.clone());
        klass.set_major_version(self.major_version.clone());
    }
}