use crate::classpath::class_path::Entry;
use std::path::PathBuf;

#[derive(Clone)]
pub struct ZipEntry {
    pub path: PathBuf,
}

impl ZipEntry {
    pub fn new(path: PathBuf) -> ZipEntry {
        ZipEntry { path }
    }
}

impl Entry for ZipEntry {
    fn read_class(&self, _class_name: String) {
        unimplemented!()
    }
}
