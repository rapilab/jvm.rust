use byteorder::{BigEndian, ByteOrder};
use jvm::instructions::opcode;
use jvm::instructions::instruction::get_instruction;
use std::borrow::Borrow;
use jvm::instructions::decoder::decoder;
use jvm::instructions::exec::InstructionExec;

fn main() {}

fn exec_bytecode_method(instr: Vec<u8>) -> Vec<Box<dyn InstructionExec>> {
    let vec = decoder(instr);
    vec
}


#[cfg(test)]
mod tests {
    use jvm::rtda::heap::runtime::Runtime;
    use crate::exec_bytecode_method;

    #[test]
    fn test_stack() {
        let mut runtime = Runtime::new();
        let string = String::from("testdata/java8/HelloWorld.Class");
        let mut class_loader = runtime.boot_loader;
        class_loader.init(string);
        for x in class_loader.jl_object_class {
            for x in x.methods {
                let ins = exec_bytecode_method(x.code.clone());
                // assert_eq!(5, ins.len());
            }
        }
    }
}