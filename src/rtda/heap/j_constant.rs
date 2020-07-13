use crate::rtda::heap::constant_member_ref::ConstantMemberRef;
use crate::oops::constant_pool::FieldRef;
use crate::classfile::class_file_parser::ClassFileParser;
use crate::oops::instanced_klass::InstanceKlass;


#[derive(Clone, Debug)]
pub enum JConstant {
    ConstantField(JField)
}

#[derive(Clone, Debug)]
pub struct JField {
    pub member_ref: ConstantMemberRef
}

impl JField {
    pub fn new(class: &InstanceKlass, cf: &ClassFileParser, field_ref: FieldRef) -> JField {
        let member_ref = ConstantMemberRef::new(class, cf, field_ref);
        JField {
            member_ref,
        }
    }
}