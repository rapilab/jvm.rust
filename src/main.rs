use jvm::instructions::instruction_factory::get_instruction;
use jvm::rtda::thread::{execute_method, create_frame, Thread};
use jvm::instructions::decoder::{decoder, Decode};
use jvm::rtda::heap::j_method::JMethod;
use jvm::instructions::exec::InstructionExec;
use jvm::rtda::frame::Frame;
use jvm::classpath::class_path::ClassPath;
use jvm::rtda::heap::runtime::Runtime;

fn main() {}

pub fn create_main_thread(jre_home: String) -> Thread {
    let cp = ClassPath::parse(String::from(jre_home), String::from("testdata/java8"));
    let runtime = Runtime::new(cp);

    let main_thread = Thread::new(runtime);

    main_thread.invoke_method_with_shim();

    main_thread
}


pub fn start_vm(jre: String) {
    let thread = create_main_thread(jre);

    let mut current_frame = thread.current_frame();
    match current_frame {
        None => {},
        Some(mut frame) => {
            let mut vec = decoder(frame.clone().method.code);
            vec[0].ins.execute(&mut frame);
        },
    }
}

#[cfg(test)]
mod tests {
    use jvm::classpath::class_path::ClassPath;
    use jvm::rtda::heap::runtime::Runtime;
    use std::fs::File;
    use zip::ZipArchive;
    use crate::{start_vm, create_main_thread};

    #[test]
    fn test_start_vm() {
        let jre_home = "/Library/Java/JavaVirtualMachines/jdk1.8.0_202.jdk/Contents/Home/jre";
        start_vm(String::from(jre_home));
    }

    #[test]
    fn test_stack() {
        let runtime = Runtime::new(ClassPath::new());
        let string = String::from("testdata/java8/HelloWorld.Class");
        let mut class_loader = runtime.boot_loader;
        class_loader.add_user_class(string);
    }

    #[test]
    fn test_main_thread() {
        let jre_home = "/Library/Java/JavaVirtualMachines/jdk1.8.0_202.jdk/Contents/Home/jre";
        create_main_thread(String::from(jre_home));
    }

    #[test]
    fn t_basic_zip() {
        let f = "testdata/java8/jar/hello.jar";
        let f = File::open(f).unwrap();
        let mut za = ZipArchive::new(f).unwrap();

        let mut can_get_hello_class = false;
        for i in 0..za.len() {
            let mut zf = za.by_index(i).unwrap();
            if zf.name().contains("HelloWorld.class") {
                // println!("{}", zf.name());
                can_get_hello_class = true;
            }
        }

        assert!(can_get_hello_class)
    }
}
