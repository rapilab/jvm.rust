use crate::classfile::class_file_stream::ClassFileStream;
use crate::oops::constant_pool::CpEntry;

const ConstantValue: &str = "ConstantValue";
const Code: &str = "Code";
const Exceptions: &str = "Exceptions";
const SourceFile: &str = "SourceFile";
const LineNumberTable: &str = "LineNumberTable";
const LocalVariableTable: &str = "local_variable_table";
const InnerClasses: &str = "InnerClasses";
const Synthetic: &str = "Synthetic";
const Deprecated: &str = "Deprecated";
const EnclosingMethod: &str = "EnclosingMethod";
const Signature: &str = "Signature";
const SourceDebugExtension: &str = "SourceDebugExtension";
const LocalVariableTypeTable: &str = "LocalVariableTypeTable";
const RuntimeVisibleAnnotations: &str = "RuntimeVisibleAnnotations";
const RuntimeInvisibleAnnotations: &str = "RuntimeInvisibleAnnotations";
const RuntimeVisibleParameterAnnotations: &str = "RuntimeVisibleParameterAnnotations";
const RuntimeInvisibleParameterAnnotation: &str = "RuntimeInvisibleParameterAnnotations";
const AnnotationDefault: &str = "AnnotationDefault";
const StackMapTable: &str = "StackMapTable";
const BootstrapMethods: &str = "BootstrapMethods";
const RuntimeVisibleTypeAnnotations: &str = "RuntimeVisibleTypeAnnotations";
const RuntimeInvisibleTypeAnnotations: &str = "RuntimeInvisibleTypeAnnotations";
const MethodParameters: &str = "MethodParameters";
const Module: &str = "Module";
const ModulePackages: &str = "ModulePackages";
const ModuleMainClass: &str = "ModuleMainClass";
const NestHost: &str = "NestHost";
const NestMembers: &str = "NestMembers";

#[derive(Clone, Debug)]
pub enum AttributeInfo {
    None(),
    ConstantValue(),
    Code(CodeAttribute),
    Exceptions(),
    SourceFile(SourceFile),
    LineNumberTable(LineNumberTableAttribute),
    LocalVariableTable(LocalVariableTable),
    InnerClasses(),
    Synthetic(),
    Deprecated(),
    EnclosingMethod(),
    Signature(),
    SourceDebugExtension(),
    LocalVariableTypeTable(),
    RuntimeVisibleAnnotations(),
    RuntimeInvisibleAnnotations(),
    RuntimeVisibleParameterAnnotations(),
    RuntimeInvisibleParameterAnnotation(),
    AnnotationDefault(),
    StackMapTable(),
    BootstrapMethods(),
    RuntimeVisibleTypeAnnotations(),
    RuntimeInvisibleTypeAnnotations(),
    MethodParameters(),
    Module(),
    ModulePackages(),
    ModuleMainClass(),
    NestHost(),
    NestMembers(),
}

#[derive(Clone, Debug)]
pub struct ExceptionTableEntry {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

#[derive(Clone, Debug)]
pub struct CodeAttribute {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<u8>,
    pub exception_table: Vec<ExceptionTableEntry>,
    pub attribute_table: Vec<AttributeInfo>,
}

#[derive(Clone, Debug)]
pub struct LineNumberTableAttribute {
    line_number_table: Vec<LineNumberTableEntry>
}

#[derive(Clone, Debug)]
struct LineNumberTableEntry {
    start_pc: u16,
    line_number: u16,
}


#[derive(Clone, Debug)]
pub struct LocalVariableTable {
    local_variable_table: Vec<LocalVariableTableEntry>
}

#[derive(Clone, Debug)]
pub struct LocalVariableTableEntry {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub index: u16,
}

#[derive(Clone, Debug)]
pub struct SourceFile {
    pub source_file_index: u16
}

impl CodeAttribute {
    pub fn new() -> CodeAttribute {
        CodeAttribute {
            max_stack: 0,
            max_locals: 0,
            code: vec![],
            exception_table: vec![],
            attribute_table: vec![],
        }
    }
}

pub fn read_exception_table(stream: &mut ClassFileStream) -> Vec<ExceptionTableEntry> {
    let mut exceptions: Vec<ExceptionTableEntry> = vec![];
    let length = stream.read_u16();
    for i in 1..length {
        let exception = ExceptionTableEntry {
            start_pc: stream.read_u16(),
            end_pc: stream.read_u16(),
            handler_pc: stream.read_u16(),
            catch_type: stream.read_u16(),
        };
        exceptions.push(exception);
    }
    exceptions
}

pub fn read_attributes(stream: &mut ClassFileStream, entries: Vec<CpEntry>) -> Vec<AttributeInfo> {
    let att_count = stream.read_u16();
    let mut attr: AttributeInfo = AttributeInfo::None();
    let mut attrs: Vec<AttributeInfo> = vec![];
    for _j in 0..att_count as usize {
        attr = read_attribute_info(stream, entries.clone());
        attrs.push(attr);
    }
    attrs
}

pub fn read_attribute_info(stream: &mut ClassFileStream, entries: Vec<CpEntry>) -> AttributeInfo {
    let attr_name_index = stream.read_u16();
    let attr_len = stream.read_u32();
    let mut attr_name: String = String::from("");
    let entry = entries[attr_name_index as usize].clone();
    if let CpEntry::Utf8 { val } = entry {
        attr_name = val;
    }

    match &attr_name[..] {
        "Code" => {
            let mut attribute = CodeAttribute {
                max_stack: stream.read_u16(),
                max_locals: stream.read_u16(),
                code: vec![],
                exception_table: vec![],
                attribute_table: vec![],
            };
            let code_length = stream.read_u32();
            attribute.code = stream.read_to_length(code_length as u16);
            attribute.exception_table = read_exception_table(stream);
            attribute.attribute_table = read_attributes(stream, entries);
            AttributeInfo::Code(attribute)
        }
        "LineNumberTable" => {
            let line_attribute = build_line_table(stream);
            AttributeInfo::LineNumberTable(line_attribute)
        }
        "LocalVariableTable" => {
            let local_vars_attr = build_local_vars_table(stream);
            AttributeInfo::LocalVariableTable(local_vars_attr)
        }
        "SourceFile" => {
            let source_file = SourceFile {
                source_file_index: stream.read_u16()
            };
            AttributeInfo::SourceFile(source_file)
        }
        _ => {
            println!("{}", attr_name);
            AttributeInfo::None()
        }
    }
}

pub fn build_local_vars_table(stream: &mut ClassFileStream) -> LocalVariableTable {
    let table_length = stream.read_u16();
    let mut local_vars_table = LocalVariableTable {
        local_variable_table: vec![]
    };

    for _i in 0..table_length as usize {
        let entry = LocalVariableTableEntry {
            start_pc: stream.read_u16(),
            length: stream.read_u16(),
            name_index: stream.read_u16(),
            descriptor_index: stream.read_u16(),
            index: stream.read_u16()
        };
        local_vars_table.local_variable_table.push(entry);
    }
    local_vars_table
}

pub fn build_line_table(stream: &mut ClassFileStream) -> LineNumberTableAttribute {
    let table_length = stream.read_u16();
    let mut line_attribute = LineNumberTableAttribute {
        line_number_table: vec![]
    };

    for _i in 0..table_length as usize {
        let entry = LineNumberTableEntry {
            start_pc: stream.read_u16(),
            line_number: stream.read_u16(),
        };
        line_attribute.line_number_table.push(entry);
    }
    line_attribute
}
