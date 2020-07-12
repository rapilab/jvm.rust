use byteorder::{BigEndian, ByteOrder};

fn main() {}

fn exec_bytecode_method(instr: Vec<u8>) {
    let mut current = 0;
    loop {
        let ops_ins = instr.get(current);
        match ops_ins {
            Some(ins) => {
                match ins {
                    0x00 => {
                        println!("NOP");
                    }
                    0x01 => {
                        println!("ACONST_NULL");
                    }
                    0x02 => {
                        println!("ICONST_M1");
                    }
                    0x03 => {
                        println!("ICONST_0");
                    }
                    0x04 => {
                        println!("ICONST_1");
                    }
                    0x12 => {
                        println!("LDC");
                    }
                    0x2a => {
                        println!("ALOAD_0")
                    }
                    0xb7 => {
                        println!("INVOKESPECIAL")
                    }
                    0xb6 => {
                        println!("INVOKEVIRTUAL")
                    }
                    0xb1 => {
                        println!("RETURN")
                    }
                    0xb2 => {
                        println!("GETSTATIC")
                    }
                    _ => {
                        println!("{}", ins);
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