use crate::classfile::attribute_info::{
    AttributeInfo, ExceptionTableEntry, LineNumberTableAttribute,
};
use crate::rtda::heap::instanced_klass::InstanceKlass;
use crate::rtda::heap::method_descriptor::MethodDescriptor;

#[derive(Debug, Clone)]
pub struct MethodData {
    pub code: Vec<u8>,
    pub exception_table: Vec<ExceptionTableEntry>,
    pub attribute_table: Vec<AttributeInfo>,
    pub parameter_annotation_data: Vec<u8>,
    pub annotation_default_data: Vec<u8>,
    pub line_num_table: LineNumberTableAttribute,
}

impl MethodData {
    pub fn new() -> MethodData {
        MethodData {
            code: vec![],
            exception_table: vec![],
            attribute_table: vec![],
            parameter_annotation_data: vec![],
            annotation_default_data: vec![],
            line_num_table: LineNumberTableAttribute::new()
        }
    }
}

#[derive(Debug, Clone)]
pub struct JMethod {
    pub name: String,
    pub klass: InstanceKlass,
    pub max_stack: u16,
    pub max_locals: u16,
    pub descriptor: MethodDescriptor,
    pub method_data: MethodData
}

impl JMethod {
    pub fn new() -> JMethod {
        JMethod {
            name: String::from(""),
            klass: InstanceKlass::new(),
            max_stack: 0,
            max_locals: 0,
            descriptor: MethodDescriptor::new(String::from("")),
            method_data: MethodData::new()
        }
    }
}
