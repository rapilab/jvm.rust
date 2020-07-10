use std::fs::File;
use std::io::Read;
use byteorder::{ByteOrder, LittleEndian};
use std::path::Path;
use jvm::util::file_to_bytes;
use jvm::klass_parser::KlassParser;

fn main() {
    let path = "testdata/HelloWorld.class".clone();
    let bytes = file_to_bytes(Path::new(path))
        .expect(&format!("Problem reading {}", path));


    let parser = KlassParser::new(bytes);
    parser.parse();
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_stack() {

    }
}