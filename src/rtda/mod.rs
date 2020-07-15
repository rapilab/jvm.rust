pub mod path_conv;
pub mod frame;
pub mod heap;
pub mod jvm_stack;
pub mod thread;

use crate::classpath::class_path::ClassPath;
use crate::rtda::heap::runtime::Runtime;
use crate::rtda::thread::Thread;

pub fn create_main_thread(jre_home: &str) -> Thread {
    let cp = ClassPath::parse(String::from(jre_home), String::from("testdata/java8"));
    let runtime = Runtime::new(cp);

    let main_thread = Thread::new(runtime);

    main_thread.invoke_method_with_shim();

    main_thread
}
