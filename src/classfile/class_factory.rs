use crate::classfile::class_file_parser::ClassFileParser;
use crate::classfile::class_file_stream::ClassFileStream;
use crate::oops::instanced_klass::InstanceKlass;

pub struct ClassFactory {}

impl ClassFactory {
    pub fn create_from_stream(stream: ClassFileStream) -> InstanceKlass {
        let mut parser = ClassFileParser::new(stream);
        let klass = parser.create_instance_klass();
        klass
    }
}
