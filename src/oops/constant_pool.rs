use std::str;

use byteorder::{ByteOrder, LittleEndian, BigEndian, ReadBytesExt};

use crate::classfile::class_file_stream::ClassFileStream;

pub const CONSTANT_UTF8: u8 = 1;
pub const CONSTANT_INTEGER: u8 = 3;
pub const CONSTANT_FLOAT: u8 = 4;
pub const CONSTANT_LONG: u8 = 5;
pub const CONSTANT_DOUBLE: u8 = 6;
pub const CONSTANT_CLASS: u8 = 7;
pub const CONSTANT_STRING: u8 = 8;
pub const CONSTANT_FIELD_REF: u8 = 9;
pub const CONSTANT_METHOD_REF: u8 = 10;
pub const CONSTANT_INTERFACE_METHOD_REF: u8 = 11;
pub const CONSTANT_NAME_AND_TYPE: u8 = 12;
pub const CONSTANT_METHOD_HANDLE: u8 = 15;
pub const CONSTANT_METHOD_TYPE: u8 = 16;
pub const CONSTANT_INVOKE_DYNAMIC: u8 = 18;
pub const CONSTANT_MODULE: u8 = 19;
pub const CONSTANT_PACKAGE: u8 = 20;
pub const CONSTANT_DYNAMIC: u8 = 17;

pub struct ConstantInfo {}

#[derive(Clone, Debug)]
pub enum CpEntry {
    Empty {},
    Utf8 { val: String },
    Integer { val: i32 },
    Float { val: f32 },
    Long { val: i64 },
    Double { val: f64 },
    Class { idx: u16 },
    String { idx: u16 },
    FieldRef { clz_idx: u16, nt_idx: u16 },
    MethodRef { class_idx: u16, name_type_idx: u16 },
    InterfaceMethodRef { clz_idx: u16, nt_idx: u16 },
    NameAndType { name_idx: u16, type_idx: u16 },
}

impl ConstantInfo {
    pub fn from(stream: &mut ClassFileStream) -> CpEntry {
        let tag = stream.get_u1();
        match tag {
            CONSTANT_UTF8 => {
                let length_bytes = stream.get_u2();
                let length = BigEndian::read_i16(&length_bytes);

                let str_bytes = stream.read_to_length(length as u16);
                let s = match str::from_utf8(&str_bytes) {
                    Ok(v) => v,
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                };
                CpEntry::Utf8 {
                    val: String::from(s)
                }
            }
            // CONSTANT_INTEGER => {}
            // CONSTANT_FLOAT => {}
            // CONSTANT_LONG => {}
            // CONSTANT_DOUBLE => {}
            CONSTANT_CLASS => {
                CpEntry::Class {
                    idx: stream.read_u16()
                }
            }
            CONSTANT_STRING => {
                CpEntry::String {
                    idx: stream.read_u16()
                }
            }
            CONSTANT_FIELD_REF => {
                CpEntry::FieldRef {
                    clz_idx: stream.read_u16(),
                    nt_idx: stream.read_u16()
                }
            }
            CONSTANT_METHOD_REF => {
                CpEntry::MethodRef {
                    class_idx: stream.read_u16(),
                    name_type_idx: stream.read_u16()
                }
            }
            // CONSTANT_INTERFACE_METHOD_REF => {}
            CONSTANT_NAME_AND_TYPE => {
                CpEntry::NameAndType {
                    name_idx: stream.read_u16(),
                    type_idx: stream.read_u16()
                }
            }
            // CONSTANT_METHOD_HANDLE => {}
            // CONSTANT_METHOD_TYPE => {}
            // CONSTANT_INVOKE_DYNAMIC => {}
            // CONSTANT_MODULE => {}
            // CONSTANT_PACKAGE => {}
            // CONSTANT_DYNAMIC => {}
            _ => {
                panic!("Unsupported Constant Pool type {} at {}", tag, stream.current)
            }
        }
    }
}

pub struct ConstantPool {}

impl ConstantPool {}