use std::path::PathBuf;
use crate::classpath::classpath::Entry;

#[derive(Clone)]
pub struct DirectoryEntry {
    pub path: PathBuf
}

impl DirectoryEntry {
    pub fn new(path: PathBuf) -> DirectoryEntry {
        DirectoryEntry {
            path
        }
    }
}

impl Entry for DirectoryEntry {
    fn read_class(&self, _class_name: String) {
        unimplemented!()
    }
}