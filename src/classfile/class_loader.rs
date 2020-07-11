use crate::classfile::class_factory::ClassFactory;
use crate::classfile::class_file_stream::ClassFileStream;
use crate::oops::instanced_klass::InstanceKlass;
use crate::classfile::class_loader_data::ClassLoaderData;
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
        let mut entry = ClassPathEntry::new();
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

    #[test]
    fn test_load_class() {
        let path = "testdata/HelloWorld.class";
        let mut class_loader = ClassLoader::new();
        let klass = class_loader.load_class(String::from(path));

        assert_eq!(0, klass.minor_version);
        assert_eq!(52, klass.major_version);
    }
}
