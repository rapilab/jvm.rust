use std::path::{Path, PathBuf};
use std::fs;
use std::io::Error;
use std::fs::{ReadDir, DirEntry};
use std::ffi::OsStr;

pub trait Entry {
    // className: fully/qualified/ClassName.class
    fn read_class(&self, class_name: String);
}

pub struct ClassPath {
    pub entries: Vec<Box<dyn Entry>>,
    pub java_home: String,
    pub user_path: String,
}

impl ClassPath {
    pub fn new(java_home: String, user_path: String) -> ClassPath {
        ClassPath {
            entries: vec![],
            java_home,
            user_path,
        }
    }

    pub fn parse(java_home: String, user_path: String) {
        let path = ClassPath::new(java_home.clone(), user_path);

        path.parse_boot_path(java_home.clone());

        path.parse_ext_path();

        path.parse_user_class_path();
    }

    pub fn parse_boot_path(&self, java_home: String) {
        let jre_path = Path::new(&java_home).join("lib");
        spread_wildcard_entry(jre_path);
    }
    pub fn parse_ext_path(&self) {}
    pub fn parse_user_class_path(&self) {}
}

pub fn spread_wildcard_entry(path: PathBuf) {
    let files = fs::read_dir(path).unwrap();
    files
        .filter_map(Result::ok)
        .filter(|d| is_jar(d))
        .for_each(|f| println!("{:?}", f));
    //
    // let names =
    //     files.filter_map(|entry| {
    //         entry.ok().and_then(|e| e.path().file_name()
    //             .and_then(|n| n.to_str().map(
    //                 |s| { String::from(s) }
    //             ))
    //         )
    //     }).collect::<Vec<String>>();
}

fn is_jar(d: &DirEntry) -> bool {
    if let Some(e) = d.path().extension() { e == "jar" } else { false }
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