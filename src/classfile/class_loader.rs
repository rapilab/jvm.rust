use crate::classfile::class_factory::ClassFactory;
use crate::classfile::class_file_stream::ClassFileStream;
use crate::oops::instanced_klass::InstanceKlass;
use crate::classfile::class_loader_data::ClassLoaderData;

pub struct ClassPathEntry {

}

impl ClassPathEntry {
    pub fn new() -> ClassPathEntry {
        ClassPathEntry {

        }
    }
    pub fn open_stream(self, file_name: String) -> ClassFileStream {
        ClassFileStream {

        }
    }
}

pub struct ClassLoader {

}

impl ClassLoader {
    pub fn load_class(&mut self, class_name: String) -> InstanceKlass {
        let file_name = self.file_name_for_class_name(class_name);
        let mut entry = ClassPathEntry::new();
        let stream = entry.open_stream(file_name);

        // let loader_data = ClassLoaderData::new();
        let klass = ClassFactory::create_from_stream(stream);
        klass
    }

    fn file_name_for_class_name(&mut self, class_name: String) -> String {
        class_name
    }
}