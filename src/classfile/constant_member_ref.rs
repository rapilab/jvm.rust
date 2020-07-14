use crate::classfile::constant_pool::MemberRef;
use crate::classfile::parsed_class::ParsedClass;
use crate::rtda::heap::instanced_klass::InstanceKlass;

#[derive(Clone, Debug)]
pub struct ConstantMemberRef {
    pub class: Box<InstanceKlass>,
    pub class_name: String,
    pub name: String,
    pub descriptor: String,
}

impl ConstantMemberRef {
    pub fn new(class: &InstanceKlass, cf: &ParsedClass, field_ref: MemberRef) -> ConstantMemberRef {
        let class_name = &class.klass_name;
        let name_type = cf.get_name_and_type(field_ref.name_type_index);
        ConstantMemberRef {
            class: Box::new(class.clone()),
            class_name: String::from(class_name),
            name: name_type.name,
            descriptor: name_type.typ,
        }
    }
}
