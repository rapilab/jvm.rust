use std::path::{Path, PathBuf};
use std::fs;
use std::io::Error;
use std::fs::{ReadDir, DirEntry};
use std::ffi::OsStr;
use crate::classpath::zip_entry::ZipEntry;

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

    pub fn parse(java_home: String, user_path: String) -> ClassPath {
        let mut classpaths = ClassPath::new(java_home.clone(), user_path);

        classpaths.parse_boot_path(java_home.clone());

        classpaths.parse_ext_path(java_home.clone());

        classpaths.parse_user_class_path();

        classpaths
    }

    pub fn parse_boot_path(&mut self, java_home: String)  {
        let jre_path = Path::new(&java_home).join("lib");
        self.spread_wildcard_entry(jre_path);
    }

    pub fn parse_ext_path(&mut self, java_home: String) {
        let jre_path = Path::new(&java_home).join("lib").join("ext");
        self.spread_wildcard_entry(jre_path);
    }

    pub fn spread_wildcard_entry(&mut self, path: PathBuf)  {
        let files = fs::read_dir(path).unwrap();
        files
            .filter_map(Result::ok)
            .filter(|d| is_jar(d))
            .for_each(|f| {
                let entry = ZipEntry::new(f.path());
                // entries.push(Box::from(entry));
                self.entries.push(Box::from(entry));
            });
    }

    pub fn parse_user_class_path(&self) {}
}

fn is_jar(d: &DirEntry) -> bool {
    if let Some(e) = d.path().extension() { e == "jar" } else { false }
}

#[cfg(test)]
mod tests {
    use crate::classpath::classpath::ClassPath;

    #[test]
    fn test_load_class() {
        // todo: change java_home to get from shell
        // "$(java -XshowSettings:properties -version 2>&1 > /dev/null | grep 'java.home' | awk '{print $3}')"
        let java_home: String = String::from("/Library/Java/JavaVirtualMachines/jdk1.8.0_202.jdk/Contents/Home/jre");
        let user_path: String = String::from("testdata/java8");
        let class_paths = ClassPath::parse(java_home, user_path);

        assert_eq!(20, class_paths.entries.len());
    }
}