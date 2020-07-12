use byteorder::{BigEndian, ByteOrder};
use jvm::instructions::opcode;
use jvm::instructions::instruction::get_instruction;
use std::borrow::Borrow;
use jvm::instructions::decoder::decoder;
use jvm::instructions::exec::InstructionExec;

fn main() {}

#[cfg(test)]
mod tests {
    use jvm::rtda::heap::runtime::Runtime;

    #[test]
    fn test_stack() {
        let mut runtime = Runtime::new();
        let string = String::from("testdata/java8/HelloWorld.Class");
        let mut class_loader = runtime.boot_loader;
        class_loader.init(string);
    }
}