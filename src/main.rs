use byteorder::{BigEndian, ByteOrder};
use jvm::instructions::opcode;

fn main() {}

fn exec_bytecode_method(instr: Vec<u8>) {
    let mut current = 0;
    loop {
        let ops_ins = instr.get(current);
        match ops_ins {
            Some(ins) => {
                match *ins {
                    opcode::OpNop => {
                        println!("NOP");
                    }
                    opcode::OpAConstNull => {
                        println!("ACONST_NULL");
                    }
                    opcode::OpIConstM1 => {
                        println!("ICONST_M1");
                    }
                    opcode::OpIConst0 => {
                        println!("ICONST_0");
                    }
                    opcode::OpIConst1 => {
                        println!("ICONST_1");
                    }
                    opcode::OpLDC => {
                        println!("LDC");
                    }
                    opcode::OpALoad0 => {
                        println!("ALOAD_0")
                    }
                    opcode::OpInvokeSpecial => {
                        println!("INVOKESPECIAL")
                    }
                    opcode::OpInvokeVirtual => {
                        println!("INVOKEVIRTUAL")
                    }
                    opcode::OpReturn => {
                        println!("RETURN")
                    }
                    opcode::OpGetStatic => {
                        println!("GETSTATIC")
                    }
                    _ => {
                        println!("{:x}", ins);
                    }
                }
            }
            _ => {
                break;
            }
        }

        current += 1;
    }
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
                exec_bytecode_method(x.code.clone())
            }
        }
    }
}