pub struct KlassParser {
    clz_read: Vec<u8>,
    filename: String,
    minor_version: u16,
    major_version: u16,
    pool_item_count: u16,
    access_flags: u16,
    this_klass: u16,
    super_klass: u16,
    interfaces: Vec<u16>,
}

impl KlassParser {
    pub fn new(buf: Vec<u8>) -> KlassParser {
        KlassParser {
            clz_read: buf,
            filename: String::from(""),
            major_version: 0,
            minor_version: 0,
            pool_item_count: 0,
            access_flags: 0,
            this_klass: 0,
            super_klass: 0,
            interfaces: Vec::new(),
        }
    }
    pub fn parse(&mut self) {
        self.parse_header();
        self.parse_constant_pool();
        self.parse_basic_type_info();
        self.parse_fields();
        self.parse_methods();
        self.parse_attributes();
    }

    fn parse_header(&mut self) {
        if self.is_klass_magic() {
            panic!(
                "Input file {} does not have correct magic number",
                self.filename
            );
        }

        self.minor_version = ((self.clz_read[4] as u16) << 8) + self.clz_read[5] as u16;
        self.major_version = ((self.clz_read[6] as u16) << 8) + self.clz_read[7] as u16;
        self.pool_item_count = ((self.clz_read[8] as u16) << 8) + self.clz_read[9] as u16;
    }

    fn parse_constant_pool(&mut self) {

    }

    fn parse_methods(&mut self) {

    }

    fn parse_attributes(&mut self) {

    }

    fn parse_fields(&mut self) {

    }

    fn parse_basic_type_info(&mut self) {

    }

    fn is_klass_magic(&mut self) -> bool {
        self.clz_read[0] != 0xca
            || self.clz_read[1] != 0xfe
            || self.clz_read[2] != 0xba
            || self.clz_read[3] != 0xbe
    }
}