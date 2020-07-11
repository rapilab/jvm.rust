use crate::classfile::class_factory::ClassFactory;
use crate::classfile::class_file_stream::ClassFileStream;

pub struct ClassLoader {

}

impl ClassLoader {
    pub fn load_class(&mut self, class_name: String) {
        file_name = self.file_name_for_class_name(class_name);
        let mut stream = ClassFileStream::new();
        stream.open_stream();
        ClassFactory::create_from_stream(stream);
    }

    fn file_name_for_class_name(&mut self, class_name: String) -> String {
        class_name
    }
}