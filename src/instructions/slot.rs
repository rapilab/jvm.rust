use crate::instructions::j_object::JObject;

#[derive(Debug, Clone)]
pub struct Slot {
    val: i64,
    obj: JObject,
}

#[derive(Debug, Clone)]
pub struct EmptySlot {}
impl EmptySlot {
    pub(crate) fn new() -> Slot {
        Slot {
            val: 0,
            obj: JObject {},
        }
    }
}

#[derive(Debug, Clone)]
pub struct IntSlot {}
impl IntSlot {
    pub(crate) fn new(n: i32) -> Slot {
        Slot {
            val: n as i64,
            obj: JObject {},
        }
    }
}
