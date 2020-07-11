use byteorder::{BigEndian, ByteOrder};

fn main() {}

fn exec_bytecode_method(instr: Vec<u8>) {
    let mut current = 0;
    loop {
        match instr.get(current) {
            Some(var) => {
                println!("{}", var);
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