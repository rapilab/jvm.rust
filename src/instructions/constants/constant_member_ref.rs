use crate::oops::instanced_klass::InstanceKlass;
use crate::classfile::class_file_parser::ClassFileParser;
use crate::oops::constant_pool::FieldRef;
use std::any::Any;

pub struct ConstantMemberRef {
    class: Box<InstanceKlass>,
    class_name: String,
    name: String,
    descriptor: String
}

impl ConstantMemberRef {
    pub fn new(class: &InstanceKlass, cf: ClassFileParser, field_ref: FieldRef) -> ConstantMemberRef {
        let class_name = &class.klass_name;
        // cf.create_instance_klass()
        cf.get_name_and_type(field_ref.nt_idx);
        ConstantMemberRef {
            class: Box::new(class.clone()),
            class_name: String::from(class_name),
            name: "".to_string(),
            descriptor: "".to_string()
        }
    }
}
