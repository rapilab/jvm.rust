use crate::classfile::class_factory::ClassFactory;
use crate::classpath::class_file_entry::ClassFileEntry;
use crate::rtda::heap::instanced_klass::InstanceKlass;

pub struct ClassLoader {
    pub jl_object_class: Vec<InstanceKlass>,
}

impl ClassLoader {
    pub fn new() -> ClassLoader {
        ClassLoader {
            jl_object_class: vec![],
        }
    }

    pub fn init(&mut self, class_name: String) {
        self.add_user_class(class_name);
    }

    fn add_user_class(&mut self, class_name: String) {
        let klass = self.build_user_class(class_name);
        self.jl_object_class.push(klass);
    }

    pub fn build_user_class(&mut self, class_name: String) -> InstanceKlass {
        let file_name = self.file_name_for_class_name(class_name);
        let entry = ClassFileEntry::new();

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
    use crate::classfile::attribute_info::AttributeInfo;
    use crate::classfile::class_loader::ClassLoader;
    use crate::rtda::heap::instanced_klass::InstanceKlass;
    use crate::classpath::class_file_entry::ClassFileEntry;
    use crate::classfile::class_file_parser::ClassFileParser;

    #[test]
    fn test_should_get_basic_info() {
        let klass = build_klass();

        assert_eq!(0, klass.minor_version);
        assert_eq!(52, klass.major_version);
        assert_eq!("HelloWorld", klass.klass_name);
        assert_eq!("java/lang/Object", klass.super_klass_name);
        assert_eq!(0, klass.interfaces.len());
        assert_eq!(2, klass.methods.len());
    }

    #[test]
    fn test_get_methods_info() {
        let klass = build_klass();


        let info = klass.methods[0].clone();
        assert_eq!(1, info.attribute_table.len());

        let method2 = klass.methods[1].clone();
        assert_eq!(1, method2.attribute_table.len());
        assert_eq!(29, klass.constant_pool.len());
    }

    fn build_klass() -> InstanceKlass {
        let path = "testdata/java8/HelloWorld.Class";
        let mut class_loader = ClassLoader::new();
        let klass = class_loader.build_user_class(String::from(path));
        klass
    }

    #[test]
    fn test_load_methods() {
        let klass = build_klass();
        let attribute_info = klass.methods[0].attribute_table[0].clone();
        match attribute_info {
            AttributeInfo::Code(code) => {
                assert_eq!(1, code.max_stack);
                assert_eq!(1, code.max_locals);
                assert_eq!(0, code.exception_table.len());
                assert_eq!(5, code.code.len());
            }
            _ => assert!(false),
        }

        let attribute_info = klass.methods[1].attribute_table[0].clone();
        match attribute_info {
            AttributeInfo::Code(code) => {
                assert_eq!(2, code.max_stack);
                assert_eq!(1, code.max_locals);
                assert_eq!(0, code.exception_table.len());
                assert_eq!(9, code.code.len());
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn should_had_source_file() {
        let klass = build_klass();
        for attr in klass.attributes {
            match attr {
                AttributeInfo::SourceFile(source) => {
                    assert!(true);
                    assert_eq!(14, source.source_file_index);
                },
                _ => assert!(false),
            }
        }
    }

    #[test]
    fn test_should_load_source_filename() {
        let klass = build_klass();
        assert_eq!("HelloWorld.java", klass.source_file);
    }

    #[test]
    fn test_should_get_correct_fields_length() {
        let klass = build_klass();
        assert_eq!(0, klass.fields.len());
    }

    #[test]
    fn should_get_array_type() {
        let path = "testdata/java8/ArraySample.class";
        let mut class_loader = ClassLoader::new();
        let klass = class_loader.build_user_class(String::from(path));
    }
}
