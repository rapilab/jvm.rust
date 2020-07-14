struct DescriptorParser {
    d: String,
}

impl DescriptorParser {
    pub fn new(str: String) -> DescriptorParser {
        DescriptorParser {
            d: "".to_string()
        }
    }
}