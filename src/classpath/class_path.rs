use crate::classpath::directory_entry::DirectoryEntry;
use crate::classpath::zip_entry::ZipEntry;
use std::fs;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};

pub trait Entry {
    // className: fully/qualified/ClassName.class
    fn read_class(&self, class_name: String);
}

pub struct ClassPath {
    pub runtime_path: Vec<Box<dyn Entry>>,
}

impl ClassPath {
    pub fn new() -> ClassPath {
        ClassPath {
            runtime_path: vec![],
        }
    }

    pub fn parse(java_home: String, user_path: String) -> ClassPath {
        let mut classpaths = ClassPath::new();

        classpaths.parse_boot_path(java_home.clone());

        classpaths.parse_ext_path(java_home.clone());

        classpaths.parse_user_class_path(user_path.clone());

        classpaths
    }

    pub fn parse_boot_path(&mut self, java_home: String) {
        let jre_path = Path::new(&java_home).join("lib");
        self.spread_wildcard_entry(jre_path);
    }

    pub fn parse_ext_path(&mut self, java_home: String) {
        let jre_path = Path::new(&java_home).join("lib").join("ext");
        self.spread_wildcard_entry(jre_path);
    }

    pub fn spread_wildcard_entry(&mut self, path: PathBuf) {
        let files = fs::read_dir(path).unwrap();
        files
            .filter_map(Result::ok)
            .filter(|d| is_dir_jar(d))
            .for_each(|f| {
                let entry = ZipEntry::new(f.path());
                self.runtime_path.push(Box::from(entry));
            });
    }

    pub fn parse_user_class_path(&mut self, path: String) {
        let is_jar = path.ends_with(".jar");
        let is_zip = path.ends_with(".zip");

        if is_jar || is_zip {
            let entry = ZipEntry::new(Path::new(&path).to_path_buf());
            self.runtime_path.push(Box::from(entry));
        } else {
            let dir_entry = DirectoryEntry::new(Path::new(&path).to_path_buf());
            self.runtime_path.push(Box::from(dir_entry));
        }
    }
}

fn is_dir_jar(d: &DirEntry) -> bool {
    if let Some(e) = d.path().extension() {
        e == "jar"
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::classpath::class_path::ClassPath;

    #[test]
    fn test_load_class() {
        // todo: change java_home to get from shell
        // "$(java -XshowSettings:properties -version 2>&1 > /dev/null | grep 'java.home' | awk '{print $3}')"
        let java_home: String =
            String::from("/Library/Java/JavaVirtualMachines/jdk1.8.0_202.jdk/Contents/Home/jre");
        let user_path: String = String::from("testdata/java8");
        let class_paths = ClassPath::parse(java_home, user_path);

        assert_eq!(21, class_paths.runtime_path.len());
    }
}
