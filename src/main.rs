fn main() {

}

#[cfg(test)]
mod tests {
    use jvm::classpath::class_path::ClassPath;
    use jvm::rtda::heap::runtime::Runtime;
    use jvm::rtda::create_main_thread;

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
        create_main_thread(jre_home);
    }
}
