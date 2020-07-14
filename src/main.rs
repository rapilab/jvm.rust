use jvm::classpath::class_path::ClassPath;
use jvm::rtda::heap::runtime::Runtime;

fn main() {
    create_main_thread("/Library/Java/JavaVirtualMachines/jdk1.8.0_202.jdk/Contents/Home/jre");
}

fn create_main_thread(jre_home: &str) {
    let cp = ClassPath::parse(String::from(jre_home), String::from("testdata/java8"));
    Runtime::new(cp);
}

#[cfg(test)]
mod tests {
    use jvm::rtda::heap::runtime::Runtime;
    use jvm::classpath::class_path::ClassPath;

    #[test]
    fn test_stack() {
        let runtime = Runtime::new(ClassPath::new());
        let string = String::from("testdata/java8/HelloWorld.Class");
        let mut class_loader = runtime.boot_loader;
        class_loader.add_user_class(string);
    }
}
