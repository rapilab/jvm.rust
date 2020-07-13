use crate::classfile::class_file_parser::ClassFileParser;
use crate::oops::constant_pool::{CpEntry, FieldRef};
use crate::oops::instanced_klass::InstanceKlass;
use crate::rtda::heap::constant_member_ref::ConstantMemberRef;

#[derive(Clone, Debug)]
pub enum JConstant {
    Empty {},
    Integer { val: i32 },
    Float { val: f32 },
    Long { val: i64 },
    Double { val: f64 },
    Class { idx: u16 },
    Utf8 { val: String },
    MethodRef { class_idx: u16, name_type_idx: u16 },

    String(JString),
    ConstantField(JField),
    ConstantInfo(CpEntry),
}

#[derive(Clone, Debug)]
pub struct JString {
    // pub class: InstanceKlass,
    pub go_str: String
}

#[derive(Clone, Debug)]
pub struct JField {
    pub member_ref: ConstantMemberRef,
}

impl JField {
    pub fn new(class: &InstanceKlass, cf: &ClassFileParser, field_ref: FieldRef) -> JField {
        let member_ref = ConstantMemberRef::new(class, cf, field_ref);
        JField { member_ref }
    }
}
