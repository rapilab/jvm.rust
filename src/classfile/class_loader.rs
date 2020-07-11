use crate::classfile::class_factory::ClassFactory;
use crate::classfile::class_file_stream::ClassFileStream;
use crate::oops::instanced_klass::InstanceKlass;

use std::str;
use std::fs;
use std::fs::File;
use std::io::Read;

pub struct ClassPathEntry {

}

impl ClassPathEntry {
    pub fn new() -> ClassPathEntry {
        ClassPathEntry {

        }
    }

    pub fn open_stream(self, filename: String) -> ClassFileStream {
        let mut f = File::open(&filename).expect("no file found");
        let metadata = fs::metadata(&filename).expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).expect("buffer overflow");

        ClassFileStream::new(buffer)
    }
}

pub struct ClassLoader {

}

impl ClassLoader {
    pub fn new() -> ClassLoader {
        ClassLoader {

        }
    }

    pub fn load_class(&mut self, class_name: String) -> InstanceKlass {
        let file_name = self.file_name_for_class_name(class_name);
        let entry = ClassPathEntry::new();
        let stream = entry.open_stream(file_name);

        let klass = ClassFactory::create_from_stream(stream);
        klass
    }

    fn file_name_for_class_name(&mut self, class_name: String) -> String {
        class_name
    }
}

#[cfg(test)]
mod tests {
    use crate::classfile::class_loader::ClassLoader;
    use crate::classfile::attribute_info::AttributeInfo;
    use byteorder::{BigEndian, ByteOrder};
    use std::str;

    #[test]
    fn test_load_class() {
        let path = "testdata/java8/HelloWorld.Class";
        let mut class_loader = ClassLoader::new();
        let klass = class_loader.load_class(String::from(path));

        assert_eq!(0, klass.minor_version);
        assert_eq!(52, klass.major_version);
        assert_eq!(klass.constant_pool_count as usize, klass.constant_pool_entries.len());
        assert_eq!("HelloWorld", klass.klass_name);
        assert_eq!("java/lang/Object", klass.super_klass_name);
        assert_eq!(0, klass.interfaces.len());
        assert_eq!(2, klass.methods.len());
        let info = klass.methods[0].clone();
        assert_eq!(1, info.attribute_table.len());

        let method2 = klass.methods[1].clone();
        assert_eq!(1, method2.attribute_table.len());

        assert_eq!(1, klass.attributes.len());
    }
}
