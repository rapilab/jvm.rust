pub trait Entry {
  // className: fully/qualified/ClassName.class
  fn read_class(&self, class_name: String);
}

pub struct ClassPath {
    pub entries: Vec<Box<dyn Entry>>,
    pub java_home: String,
    pub user_path: String
}

impl ClassPath {
    pub fn new(java_home: String, user_path: String) -> ClassPath {
        ClassPath {
            entries: vec![],
            java_home,
            user_path
        }
    }

    pub fn parse(java_home: String, user_path: String) {
        let path = ClassPath::new(java_home, user_path);
        path.parse_boot_path();
        path.parse_ext_path();
        path.parse_user_class_path();
    }

    pub fn parse_boot_path(&self) {}
    pub fn parse_ext_path(&self) {}
    pub fn parse_user_class_path(&self) {}
}


#[cfg(test)]
mod tests {
    use crate::classpath::classpath::ClassPath;

    #[test]
    fn test_load_class() {
        let java_home: String = String::from("/Library/Java/JavaVirtualMachines/jdk1.8.0_202.jdk/Contents/Home/jre");
        let user_path: String = String::from("testdata/java8");
        let class_parse = ClassPath::parse(java_home, user_path);
    }
}