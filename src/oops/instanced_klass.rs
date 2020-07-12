use byteorder::{ByteOrder, BigEndian};
use crate::oops::constant_pool::CpEntry;
use crate::classfile::member_info::MemberInfo;
use crate::classfile::attribute_info::{AttributeInfo, ExceptionTableEntry, LineNumberTableAttribute};


#[derive(Debug, Clone)]
pub struct InstanceKlass {
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool_count: u8,
    pub constant_pool_entries: Vec<CpEntry>,
    pub klass_name: String,
    pub super_klass_name: String,
    pub interfaces: Vec<String>,
    pub methods: Vec<JMethod>,
    pub attributes: Vec<AttributeInfo>,
    pub source_file: String,
}

#[derive(Debug, Clone)]
pub struct JMethod {
    pub name: String,
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<u8>,
    pub exception_table: Vec<ExceptionTableEntry>,
    pub attribute_table: Vec<AttributeInfo>,
    pub parameter_annotation_data: Vec<u8>,
    pub annotation_default_data: Vec<u8>,
    pub line_num_table: LineNumberTableAttribute,

}

impl JMethod {
    pub fn new() -> JMethod {
        JMethod {
            name: String::from(""),
            max_stack: 0,
            max_locals: 0,
            code: vec![],
            exception_table: vec![],
            attribute_table: vec![],
            parameter_annotation_data: vec![],
            annotation_default_data: vec![],
            line_num_table: LineNumberTableAttribute::new()
        }
    }
}

impl InstanceKlass {
    pub fn new() -> InstanceKlass {
        InstanceKlass {
            minor_version: 0,
            major_version: 0,
            constant_pool_count: 0,
            constant_pool_entries: vec![],
            klass_name: String::from(""),
            super_klass_name: String::from(""),
            interfaces: vec![],
            methods: vec![],
            attributes: vec![],
            source_file: String::from("")
        }
    }

    pub fn set_minor_version(&mut self, vector: Vec<u8>) {
        self.minor_version = BigEndian::read_u16(&vector);
    }

    pub fn set_major_version(&mut self, vector: Vec<u8>) {
        self.major_version = BigEndian::read_u16(&vector);
    }

    pub fn set_super_name(&mut self, index: u16) {
        let entry = self.constant_pool_entries[index as usize].clone();
        self.super_klass_name = self.get_string_from_cp(entry);
    }

    pub fn set_class_name(&mut self, index: u16) {
        self.klass_name = self.get_class_name(index);
    }

    fn get_class_name(&mut self, index: u16) -> String {
        let entry = self.constant_pool_entries[index as usize].clone();
        self.get_string_from_cp(entry)
    }

    pub fn set_methods(&mut self, methods: Vec<MemberInfo>) {
        for x in methods {
            let mut j_method = JMethod::new();
            j_method.name = self.klass_name.clone();
            j_method.attribute_table = x.attribute_table.clone();

            for j in x.attribute_table {
                match j {
                    AttributeInfo::Code(code) => {
                        j_method.max_stack = code.max_stack;
                        j_method.code = code.code;
                        j_method.max_locals = code.max_locals;
                        j_method.exception_table = code.exception_table;
                    },
                    _ => {}
                }
            }
            if j_method.name != String::from("") {
                self.methods.push(j_method);
            }
        }
    }

    pub fn set_attributes(&mut self, attributes: Vec<AttributeInfo>) {
        self.attributes = attributes;
        for x in self.attributes.to_vec() {
            match x {
                AttributeInfo::SourceFile(source_file) => {
                    let string = self.get_string_by_index(source_file.source_file_index);
                    self.source_file = string;
                },
                _ => {}
            }
        }
    }

    pub fn set_interfaces(&mut self, interfaces: Vec<u16>) {
        let mut results: Vec<String> = vec![];
        for x in interfaces {
            results.push(self.get_class_name(x));
        }
        self.interfaces = results
    }

    fn get_string_from_cp(&mut self, entry: CpEntry) -> String {
        let mut class_name: String = String::from("");
        if let CpEntry::Class { idx } = entry {
            class_name = self.get_string_by_index(idx)
        }
        class_name
    }

    fn get_string_by_index(&mut self, idx: u16) -> String {
        let mut str: String = String::from("");
        let name = self.constant_pool_entries[idx as usize].clone();
        if let CpEntry::Utf8 { val } = name {
            str = val;
        }
        str
    }
}