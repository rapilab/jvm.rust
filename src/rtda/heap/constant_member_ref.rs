use crate::oops::instanced_klass::InstanceKlass;
use crate::classfile::class_file_parser::ClassFileParser;
use crate::oops::constant_pool::FieldRef;

#[derive(Clone, Debug)]
pub struct ConstantMemberRef {
    pub class: Box<InstanceKlass>,
    pub class_name: String,
    pub name: String,
    pub descriptor: String
}

impl ConstantMemberRef {
    pub fn new(class: &InstanceKlass, cf: &ClassFileParser, field_ref: FieldRef) -> ConstantMemberRef {
        let class_name = &class.klass_name;
        let name_type = cf.get_name_and_type(field_ref.nt_idx);
        ConstantMemberRef {
            class: Box::new(class.clone()),
            class_name: String::from(class_name),
            name: name_type.name,
            descriptor: name_type.typ
        }
    }
}
