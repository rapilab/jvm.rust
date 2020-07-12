use crate::classfile::class_file_stream::ClassFileStream;
use std::fs::File;
use std::fs;
use std::io::Read;

pub struct ClassFileEntry {

}

impl ClassFileEntry {
    pub fn new() -> ClassFileEntry {
        ClassFileEntry {

        }
    }

    pub fn open_stream(self, filename: String) -> ClassFileStream {
        let mut f = File::open(&filename).expect("no file found");
        let metadata = fs::metadata(&filename).expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).expect("buffer overflow");

        ClassFileStream::new(buffer)
    }
}