use crate::classfile::parsed_class::ParsedClass;
use crate::classfile::class_file_stream::ClassFileStream;
use crate::rtda::heap::instanced_klass::InstanceKlass;

pub struct ClassFactory {}

impl ClassFactory {
    pub fn create_from_stream(stream: ClassFileStream) -> InstanceKlass {
        let mut parser = ParsedClass::new(stream);
        let klass = parser.create_instance_klass();
        klass
    }
}
