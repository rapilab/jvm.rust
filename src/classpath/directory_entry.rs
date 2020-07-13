use crate::classpath::class_path::Entry;
use std::path::PathBuf;

#[derive(Clone)]
pub struct DirectoryEntry {
    pub path: PathBuf,
}

impl DirectoryEntry {
    pub fn new(path: PathBuf) -> DirectoryEntry {
        DirectoryEntry { path }
    }
}

impl Entry for DirectoryEntry {
    fn read_class(&self, _class_name: String) {
        unimplemented!()
    }
}
