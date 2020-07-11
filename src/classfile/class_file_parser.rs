use crate::classfile::class_file_stream::ClassFileStream;
use crate::oops::instanced_klass::InstanceKlass;
use std::borrow::Borrow;

pub struct ClassFileParser {
    stream: ClassFileStream,
    major_version: u16,
    minor_version: u16,
    this_class_index: u16,
    super_class_index: u16,
    itfs_len: u16,
    java_fields_count: u16

}


fn parse_stream(stream: ClassFileStream) {

}

impl ClassFileParser {
    pub fn new(stream: ClassFileStream) -> ClassFileParser {
        parse_stream(stream.clone());
        ClassFileParser {
            stream,
            major_version: 0,
            minor_version: 0,
            this_class_index: 0,
            super_class_index: 0,
            itfs_len: 0,
            java_fields_count: 0
        }
    }

    pub fn create_instance_klass(&mut self) -> InstanceKlass {
        let mut klass = InstanceKlass::new();
        self.fill_instance_klass(klass.clone());
        klass
    }

    fn fill_instance_klass(&mut self, mut klass: InstanceKlass) {
        // klass.set_class_loader_data();
        klass.set_name();
    }
}