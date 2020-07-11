use crate::classfile::class_file_stream::ClassFileStream;
use crate::oops::instanced_klass::InstanceKlass;
use std::borrow::Borrow;

pub struct ClassFileParser {
    major_version: u16,
    minor_version: u16,
    this_class_index: u16,
    super_class_index: u16,
    itfs_len: u16,
    java_fields_count: u16

}


fn parse_stream(file_parser: &ClassFileParser, mut stream: ClassFileStream) {
    stream.get_u4();
}

impl ClassFileParser {
    pub fn new(stream: ClassFileStream) -> ClassFileParser {
        let file_parser = ClassFileParser {
            major_version: 0,
            minor_version: 0,
            this_class_index: 0,
            super_class_index: 0,
            itfs_len: 0,
            java_fields_count: 0
        };
        parse_stream(file_parser.borrow(), stream.clone());

        file_parser
    }

    pub fn create_instance_klass(&mut self) -> InstanceKlass {
        let mut klass = InstanceKlass::new();
        self.fill_instance_klass(klass.clone());
        klass
    }

    fn fill_instance_klass(&mut self, mut klass: InstanceKlass) {
        klass.set_name();
    }
}