use crate::classfile::class_file_stream::ClassFileStream;
use crate::oops::constant_pool::CpEntry;

const ConstantValue: &str = "ConstantValue";
const Code: &str = "Code";
const Exceptions: &str = "Exceptions";
const SourceFile: &str = "SourceFile";
const LineNumberTable: &str = "LineNumberTable";
const LocalVariableTable: &str = "LocalVariableTable";
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
    SourceFile(),
    LineNumberTable(),
    LocalVariableTable(),
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
    max_stack: u16,
    max_locals: u16,
    code: Vec<u8>,
    exception_table: Vec<ExceptionTableEntry>,
    attribute_table: Vec<ExceptionTableEntry>,
}

impl CodeAttribute {
    pub fn new() -> CodeAttribute {
        CodeAttribute {
            max_stack: 0,
            max_locals: 0,
            code: vec![],
            exception_table: vec![],
            attribute_table: vec![]
        }
    }
}

pub fn read_attribute_info(stream: &mut ClassFileStream, entries: Vec<CpEntry>) -> AttributeInfo {
    let attr_name_index = stream.read_u16();
    let attrLen = stream.read_u32();
    let mut attr_name: String = String::from("");
    let entry = entries[attr_name_index as usize].clone();
    if let CpEntry::Utf8 { val } = entry {
        attr_name = val;
    }
    match &attr_name[..] {
        "Code" => {
            AttributeInfo::Code(CodeAttribute::new())
        }
        _ => {
            println!("{}", attr_name);
            AttributeInfo::None()
        }
    }
}
