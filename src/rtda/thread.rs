use crate::instructions::decoder::decoder;
use crate::instructions::exec::InstructionExec;
use crate::rtda::frame::Frame;
use crate::rtda::heap::j_method::JMethod;
use crate::rtda::j_stack::JStack;
use std::borrow::Borrow;
use crate::rtda::heap::runtime::Runtime;

#[derive(Debug, Clone)]
pub struct JThread {
    pub stack: Box<JStack>,
    pub runtime: Box<Runtime>,
}

impl JThread {
    pub fn new(runtime: Runtime) -> JThread {
        JThread {
            runtime: Box::from(runtime),
            stack: Box::from(JStack::new(0)),
        }
    }

    pub fn push_frame(&mut self, frame: &Frame) {
        self.stack.push(frame)
    }

    pub fn new_frame(self, method: JMethod) -> Frame {
        Frame::new(Box::from(self), method)
    }

    pub fn invoke_method_with_shim(&self) {}
}

pub fn execute_method(frame: &mut Frame, instr: Vec<u8>) -> Vec<Box<dyn InstructionExec>> {
    let _length = instr.len();
    let mut vec = decoder(instr.clone());
    for i in 0..vec.len() {
        vec[i].execute(frame);
    }

    vec
}

pub fn create_frame(method: &JMethod, thread: &mut JThread) -> Frame {
    let frame = thread.clone().new_frame(method.clone());
    thread.push_frame(frame.borrow());
    frame
}

#[cfg(test)]
mod tests {
    use crate::rtda::heap::runtime::Runtime;
    use crate::rtda::thread::{create_frame, execute_method, JThread};
    use crate::classpath::class_path::ClassPath;
    use crate::rtda::create_main_thread;

    #[test]
    fn test_frame() {
        let runtime = Runtime::new(ClassPath::new());
        let string = String::from("testdata/java8/HelloWorld.Class");
        let mut class_loader = runtime.boot_loader;
        class_loader.add_user_class(string);

        let klass = class_loader.jl_object_class.get(0).unwrap();
        let second = klass.methods.get(1).unwrap();
        let first = klass.methods.get(0).unwrap();

        let jre_home = "/Library/Java/JavaVirtualMachines/jdk1.8.0_202.jdk/Contents/Home/jre";
        let mut thread = create_main_thread(jre_home);

        let mut frame1 = create_frame(first, &mut thread);
        let first_execs = execute_method(&mut frame1, first.clone().code);
        assert_eq!(5, first_execs.len());

        let mut frame2 = create_frame(second, &mut thread);
        let execs = execute_method(&mut frame2, second.clone().code);
        assert_eq!(9, execs.len());
    }
}
