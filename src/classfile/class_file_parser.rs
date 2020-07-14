use byteorder::{BigEndian, ByteOrder};

use crate::classfile::attribute_info::{read_attribute_info, read_attributes, AttributeInfo};
use crate::classfile::class_file_stream::ClassFileStream;
use crate::classfile::constant_pool::{ConstantInfo, CpEntry};
use crate::classfile::member_info::MemberInfo;
use crate::rtda::heap::instanced_klass::InstanceKlass;

pub struct ClassFileParser {
    major_version: Vec<u8>,
    minor_version: Vec<u8>,
    constant_pool_count: u8,
    constant_pool_entries: Vec<CpEntry>,
    access_flags: Vec<u8>,
    this_class_index: u16,
    super_class_index: u16,
    interface_count: u16,
    interfaces: Vec<u16>,
    field_count: u16,
    fields: Vec<MemberInfo>,
    method_count: u16,
    methods: Vec<MemberInfo>,
    attr_count: u16,
    attributes: Vec<AttributeInfo>,
}

#[derive(Clone, Debug)]
pub struct NameAndType {
    pub(crate) name: String,
    pub(crate) typ: String,
}

impl NameAndType {
    pub fn new() -> NameAndType {
        NameAndType {
            name: String::from(""),
            typ: String::from(""),
        }
    }
}

fn to_u32(slice: &[u8]) -> u32 {
    slice
        .iter()
        .fold((0, 1), |(acc, mul), &bit| {
            (acc + (mul * (1 & bit as u32)), mul.wrapping_add(mul))
        })
        .0
}

fn is_klass_magic(clz_read: Vec<u8>) -> bool {
    clz_read[0] != 0xca || clz_read[1] != 0xfe || clz_read[2] != 0xba || clz_read[3] != 0xbe
}

impl ClassFileParser {
    pub fn new(stream: ClassFileStream) -> ClassFileParser {
        let mut file_parser = ClassFileParser {
            major_version: vec![0; 2],
            minor_version: vec![0; 2],
            constant_pool_count: 0,
            constant_pool_entries: vec![],
            access_flags: vec![0; 2],
            this_class_index: 0,
            super_class_index: 0,
            interface_count: 0,
            interfaces: vec![],
            field_count: 0,
            fields: vec![],
            method_count: 0,
            methods: vec![],
            attr_count: 0,
            attributes: vec![],
        };
        file_parser.parse_stream(stream.clone());

        file_parser
    }

    fn parse_stream(&mut self, mut stream: ClassFileStream) {
        let magic = stream.get_u4();
        if is_klass_magic(magic) {
            panic!("Input file {} does not have correct magic number")
        }

        self.minor_version = stream.get_u2();
        self.major_version = stream.get_u2();

        self.constant_pool_count = BigEndian::read_u16(&stream.get_u2()) as u8;
        self.constant_pool_entries =
            self.parse_constant_pool(&mut stream, self.constant_pool_count as usize);

        self.access_flags = stream.get_u2();
        self.this_class_index = stream.read_u16();
        self.super_class_index = stream.read_u16();

        self.interface_count = stream.read_u16();
        self.interfaces = self.parse_interfaces(&mut stream, self.interface_count as usize);

        self.field_count = stream.read_u16();
        self.fields = self.parse_fields(&mut stream, self.field_count as usize);

        self.method_count = stream.read_u16();
        self.methods = self.parse_fields(&mut stream, self.method_count as usize);

        self.attributes = read_attributes(&mut stream, self.constant_pool_entries.clone());
    }

    fn parse_fields(&mut self, stream: &mut ClassFileStream, size: usize) -> Vec<MemberInfo> {
        let mut members = vec![];
        for _i in 0..size {
            let mut member = MemberInfo {
                access_flags: stream.read_u16(),
                name_index: stream.read_u16(),
                descriptor_index: stream.read_u16(),
                attribute_table: vec![],
            };

            let att_count = stream.read_u16();
            for _j in 0..att_count as usize {
                let attr = read_attribute_info(stream, self.constant_pool_entries.clone());
                member.attribute_table.push(attr);
            }
            members.push(member);
        }
        members
    }

    fn parse_interfaces(&mut self, stream: &mut ClassFileStream, size: usize) -> Vec<u16> {
        let mut results: Vec<u16> = vec![];
        for _i in 1..size {
            results.push(stream.read_u16())
        }
        results
    }

    fn parse_constant_pool(&mut self, stream: &mut ClassFileStream, size: usize) -> Vec<CpEntry> {
        let mut entries: Vec<CpEntry> = vec![];
        entries.push(CpEntry::Empty {});
        // The constant_pool table is indexed from 1
        for _i in 1..size {
            entries.push(ConstantInfo::from(stream));
        }
        entries
    }

    pub fn create_instance_klass(&mut self) -> InstanceKlass {
        let mut klass = InstanceKlass::new();
        self.fill_instance_klass(&mut klass);
        klass
    }

    pub fn get_constant_info(&self, cp_index: u16) -> CpEntry {
        self.constant_pool_entries[cp_index as usize].clone()
    }

    pub fn get_utf8(&self, cp_index: u16) -> String {
        if cp_index == 0 {
            return String::from("");
        }
        match self.get_constant_info(cp_index) {
            CpEntry::Utf8 { val } => val,
            _ => String::from(""),
        }
    }

    pub fn get_name_and_type(&self, cp_index: u16) -> NameAndType {
        let mut and_type = NameAndType::new();
        if cp_index < 0 {
            return and_type;
        }

        match self.get_constant_info(cp_index) {
            CpEntry::NameAndType { name_idx, type_idx } => {
                and_type.name = self.get_utf8(name_idx);
                and_type.typ = self.get_utf8(type_idx);
            }
            _ => {}
        }

        and_type
    }

    fn fill_instance_klass(&mut self, klass: &mut InstanceKlass) {
        klass.set_origin_pool_entries(self.constant_pool_entries.clone());
        klass.set_minor_version(self.minor_version.clone());
        klass.set_major_version(self.major_version.clone());

        klass.fill_class_name(self.this_class_index);
        klass.fill_super_name(self.super_class_index);
        klass.fill_interfaces(self.interfaces.clone());
        klass.fill_fields(self.fields.clone());
        klass.fill_attributes(self.attributes.clone());

        // should before set methods
        klass.fill_pool(self);

        klass.fill_methods(self.methods.clone());
    }
}

#[cfg(test)]
mod tests {
    use crate::classfile::class_file_parser::ClassFileParser;
    use crate::classpath::class_file_entry::ClassFileEntry;

    #[test]
    fn should_eq_count_entries_length() {
        let entry = ClassFileEntry::new();
        let stream = entry.open_stream(String::from("testdata/java8/HelloWorld.Class"));
        let mut parser = ClassFileParser::new(stream);
        assert_eq!(
            parser.constant_pool_count,
            parser.constant_pool_entries.len() as u8
        );
    }
}
