pub mod frame;
pub mod heap;
pub mod jvm_stack;
pub mod thread;

use crate::rtda::thread::JThread;
use crate::classpath::class_path::ClassPath;
use crate::rtda::heap::runtime::Runtime;

pub fn create_main_thread(jre_home: &str) -> JThread {
    let cp = ClassPath::parse(String::from(jre_home), String::from("testdata/java8"));
    let runtime = Runtime::new(cp);
    let main_thread = JThread::new(runtime);
    main_thread.invoke_method_with_shim();

    main_thread
}
