use crate::instructions::j_object::JObject;

pub struct Slot {
    val: i64,
    obj: JObject,
}

pub struct EmptySlot {}
impl EmptySlot {
    pub(crate) fn new() -> Slot {
        Slot {
            val: 0,
            obj: JObject {},
        }
    }
}