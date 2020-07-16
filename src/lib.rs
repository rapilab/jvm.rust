use crate::classpath::class_path::ClassPath;
use crate::instructions::decoder::decoder;
use crate::rtda::heap::runtime::Runtime;
use crate::rtda::thread::Thread;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

pub mod classfile;
pub mod classpath;
pub mod instructions;
pub mod rtda;

pub fn create_main_thread(jre_home: String, source: String) -> Rc<RefCell<Thread>> {
    let cp = ClassPath::parse(String::from(jre_home), String::from(source));
    let runtime = Runtime::new(cp);

    let mut main_thread = Rc::new(RefCell::new(Thread::new(runtime)));

    main_thread.borrow_mut().invoke_method_with_shim();

    main_thread
}

pub fn start_vm(jre: String, source: String) {
    let thread = create_main_thread(jre, source);
    // loop here
    looper(thread)
}

fn looper(thread: Rc<RefCell<Thread>>) {
    let current_frame = thread.borrow_mut().current_frame();
    match current_frame {
        None => {}
        Some(mut frame) => {
            let mut vec = decoder(frame.clone().method.method_data.code);
            vec[0].ins.execute(&mut frame);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use zip::ZipArchive;

    use crate::classpath::class_path::ClassPath;
    use crate::rtda::heap::runtime::Runtime;
    use crate::{create_main_thread, start_vm};

    #[test]
    fn test_start_vm() {
        let source = String::from("testdata/java8/HelloWorld.Class");
        let jre_home = "/Library/Java/JavaVirtualMachines/jdk1.8.0_202.jdk/Contents/Home/jre";
        start_vm(String::from(jre_home), source);
    }

    #[test]
    fn test_stack() {
        let runtime = Runtime::new(ClassPath::new());
        let source = String::from("testdata/java8/HelloWorld.Class");
        let mut class_loader = runtime.boot_loader;
        class_loader.add_user_class(source);
    }

    #[test]
    fn test_main_thread() {
        let source = String::from("testdata/java8/HelloWorld.Class");
        let jre_home = "/Library/Java/JavaVirtualMachines/jdk1.8.0_202.jdk/Contents/Home/jre";
        create_main_thread(String::from(jre_home), source);
    }

    #[test]
    fn t_basic_zip() {
        let f = "testdata/java8/jar/hello.jar";
        let f = File::open(f).unwrap();
        let mut za = ZipArchive::new(f).unwrap();

        let mut can_get_hello_class = false;
        for i in 0..za.len() {
            let zf = za.by_index(i).unwrap();
            if zf.name().contains("HelloWorld.class") {
                // println!("{}", zf.name());
                can_get_hello_class = true;
            }
        }

        assert!(can_get_hello_class)
    }
}
