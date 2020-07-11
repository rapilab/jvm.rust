pub struct ConstantStringInfo {
    pub string_index: u16,
}

impl ConstantStringInfo {
    pub fn new() -> ConstantStringInfo {
        ConstantStringInfo {
            string_index: 0
        }
    }
}