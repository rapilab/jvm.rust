pub struct KlassParser {
    clz_read: Vec<u8>,
    filename: String,
    major: u16,
    minor: u16,
    pool_item_count: u16,
}

impl KlassParser {
    pub fn new(buf: Vec<u8>) -> KlassParser {
        KlassParser {
            clz_read: buf,
            filename: String::from(""),
            major: 0,
            minor: 0,
            pool_item_count: 0,
        }
    }
    pub fn parse(&self) {
        self.parse_header();
    }

    fn parse_header(&self) {
        if self.clz_read[0] != 0xca
            || self.clz_read[1] != 0xfe
            || self.clz_read[2] != 0xba
            || self.clz_read[3] != 0xbe
        {
            panic!(
                "Input file {} does not have correct magic number",
                self.filename
            );
        }

        self.minor = ((self.clz_read[4] as u16) << 8) + self.clz_read[5] as u16;
        self.major = ((self.clz_read[6] as u16) << 8) + self.clz_read[7] as u16;
        self.pool_item_count = ((self.clz_read[8] as u16) << 8) + self.clz_read[9] as u16;
    }
}