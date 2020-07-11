use crate::classfile::attribute_info::AttributeInfo;

pub struct MemberInfo {
    pub(crate) access_flags:u16,
    pub(crate) name_index:u16,
    pub(crate) descriptor_index:u16,
    pub(crate) attribute_table: Vec<AttributeInfo>
}
