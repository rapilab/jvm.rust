use crate::classfile::class_file_stream::ClassFileStream;
use byteorder::{LittleEndian, ByteOrder};
use crate::classfile::cp_method_ref_info::ConstantMemberRefInfo;

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

impl ConstantInfo {
    pub fn from(stream: &mut ClassFileStream) {
        let tag = stream.get_u1();
        match tag {
            // CONSTANT_UTF8 => {}
            // CONSTANT_INTEGER => {}
            // CONSTANT_FLOAT => {}
            // CONSTANT_LONG => {}
            // CONSTANT_DOUBLE => {}
            // CONSTANT_CLASS => {}
            // CONSTANT_STRING => {}
            // CONSTANT_FIELD_REF => {}
            CONSTANT_METHOD_REF => {
                let mut info = ConstantMemberRefInfo::new();
                info.class_index = stream.read_u16();
                info.name_and_type_index = stream.read_u16();
            }
            // CONSTANT_INTERFACE_METHOD_REF => {}
            // CONSTANT_NAME_AND_TYPE => {}
            // CONSTANT_METHOD_HANDLE => {}
            // CONSTANT_METHOD_TYPE => {}
            // CONSTANT_INVOKE_DYNAMIC => {}
            // CONSTANT_MODULE => {}
            // CONSTANT_PACKAGE => {}
            // CONSTANT_DYNAMIC => {}
            _ => {
                println!("{}", tag);
            }
        }
    }
}

pub struct ConstantPool {}

impl ConstantPool {}