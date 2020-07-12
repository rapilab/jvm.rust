// pub trait Entry {
//   // className: fully/qualified/ClassName.class
//   fn read_class(class_name: String);
// }

pub struct Entry {}

pub struct ClassPath {
    entries: Vec<Entry>
}

impl ClassPath {
    pub fn new() -> ClassPath {
        ClassPath {
            entries: vec![]
        }
    }

    pub fn parse() {
        let path = ClassPath::new();
        path.parse_boot_path();
        path.parse_ext_path();
        path.parse_user_class_path();
    }

    pub fn parse_boot_path(&self) {}
    pub fn parse_ext_path(&self) {}
    pub fn parse_user_class_path(&self) {}
}