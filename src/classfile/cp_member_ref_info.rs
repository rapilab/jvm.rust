pub struct ConstantMemberRefInfo {
    pub class_index: u16,
    pub name_and_type_index: u16,
    pub member_type: String
}

impl ConstantMemberRefInfo {
    pub fn new() -> ConstantMemberRefInfo {
        ConstantMemberRefInfo {
            class_index: 0,
            name_and_type_index: 0,
            member_type: String::from("")
        }
    }
}