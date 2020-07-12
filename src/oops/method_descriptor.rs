type TypeDescriptor = String;

#[derive(Debug, Clone)]
pub struct MethodDescriptor {
    pub parameter_types: Vec<TypeDescriptor>,
    pub return_type: Option<TypeDescriptor>,
}

impl MethodDescriptor {
    pub fn new() -> MethodDescriptor {
        MethodDescriptor {
            parameter_types: vec![],
            return_type: None
        }
    }
}

