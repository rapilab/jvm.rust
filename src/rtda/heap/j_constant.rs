use crate::classfile::constant_member_ref::ConstantMemberRef;
use crate::classfile::constant_pool::{CpEntry, MemberRef};
use crate::classfile::parsed_class::ParsedClass;
use crate::rtda::heap::instanced_klass::InstanceKlass;

#[derive(Clone, Debug)]
pub enum JConstant {
    Empty {},
    Integer { val: i32 },
    Float { val: f32 },
    Long { val: i64 },
    Double { val: f64 },
    Class { idx: u16 },
    Utf8 { val: String },
    String(JString),

    ConstantMethodRef(JMethodRef),
    ConstantField(JField),
    ConstantInfo(CpEntry),
}

#[derive(Clone, Debug)]
pub struct JMethodRef {
    // pub resolved: JMethod,
    pub member_ref: ConstantMemberRef,
}

impl JMethodRef {
    pub fn new(class: &InstanceKlass, cf: &ParsedClass, method_ref: MemberRef) -> JMethodRef {
        let member_ref = ConstantMemberRef::new(class, cf, method_ref);
        JMethodRef { member_ref }
    }
}

#[derive(Clone, Debug)]
pub struct JString {
    // pub class: InstanceKlass,
    pub go_str: String,
}

#[derive(Clone, Debug)]
pub struct JField {
    pub member_ref: ConstantMemberRef,
}

impl JField {
    pub fn new(class: &InstanceKlass, cf: &ParsedClass, field_ref: MemberRef) -> JField {
        let member_ref = ConstantMemberRef::new(class, cf, field_ref);
        JField { member_ref }
    }
}
