fn main() {}

#[cfg(test)]
mod tests {
    use jvm::rtda::heap::runtime::Runtime;

    #[test]
    fn test_stack() {
        let runtime = Runtime::new();
        let string = String::from("testdata/java8/HelloWorld.Class");
        let mut class_loader = runtime.boot_loader;
        class_loader.init(string);
    }
}