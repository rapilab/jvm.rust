use crate::classfile::class_file_stream::ClassFileStream;
use crate::oops::instanced_klass::InstanceKlass;

pub struct ClassFactory {

}

impl ClassFactory {
    pub fn create_from_stream(stream: ClassFileStream) -> InstanceKlass {
        let klass = InstanceKlass::new();
        klass
    }
}