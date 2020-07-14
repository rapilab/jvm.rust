use crate::rtda::heap::class_loader::ClassLoader;

pub struct Runtime {
    pub boot_loader: ClassLoader,
}

impl Runtime {
    pub fn new() -> Runtime {
        Runtime {
            boot_loader: ClassLoader::new(),
        }
    }
}
