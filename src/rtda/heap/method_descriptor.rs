#[derive(Debug, Clone)]
pub struct TypeDescriptor {
    str: String,
}

impl TypeDescriptor {
    pub fn new(str: String) -> TypeDescriptor {
        TypeDescriptor { str }
    }
}

#[derive(Debug, Clone)]
pub struct MethodDescriptor {
    pub text: String,
    pub parameter_types: Vec<TypeDescriptor>,
    pub return_type: TypeDescriptor,
}

impl MethodDescriptor {
    pub fn new(str: String) -> MethodDescriptor {
        MethodDescriptor {
            text: str,
            parameter_types: vec![],
            return_type: TypeDescriptor::new(String::from("")),
        }
    }

    pub fn parse(&mut self) -> &mut MethodDescriptor {
        let param_types = self.parse_param_types();
        let return_type = self.parse_return_type();

        self.parameter_types = param_types;
        self.return_type = return_type;

        self
    }

    pub fn parse_param_types(&mut self) -> Vec<TypeDescriptor> {
        let mut params: Vec<TypeDescriptor> = vec![];
        if self.text.len() == 0 && !self.text.starts_with("(") {
            return params;
        }

        self.text = self.text[1..].to_string();

        let mut not_finish_parse = true;
        while not_finish_parse {
            let option = self.parse_field_type();
            match option {
                None => not_finish_parse = false,
                Some(desc) => params.push(desc),
            }
        }

        if self.text.len() == 0 && !self.text.ends_with(")") {
            return vec![];
        }

        self.text = self.text[1..].to_string();
        params
    }

    pub fn parse_field_type(&mut self) -> Option<TypeDescriptor> {
        if self.text.len() > 0 {
            let string = get_char_by_index(self.text.clone(), 0);
            let x: &str = string.as_str();
            match x {
                "B" | "C" | "D" | "F" | "I" | "J" | "S" | "Z" => {
                    let t = &self.text[0..1];
                    let descriptor = TypeDescriptor::new(String::from(t));
                    return Option::from(descriptor);
                }
                "L" => {
                    let len = self.text.len();
                    for i in 0..len - 1 {
                        let text = self.text.clone();
                        if get_char_by_index(text.clone(), i) == ';'.to_string() {
                            let x = &text[0..i + 1];
                            let s = &text[i + 1..len];
                            self.text = s.to_string();
                            let descriptor = TypeDescriptor::new(String::from(x));
                            return Some(descriptor);
                        }
                    }
                }
                "[" => {}
                _ => return None,
            }
        }

        None
    }

    pub fn parse_object_type(&self) {}

    pub fn parse_return_type(&mut self) -> TypeDescriptor {
        match self.parse_field_type() {
            None => {
                self.text = String::from("V");
                TypeDescriptor::new(String::from("V"))
            }
            Some(val) => val,
        }
    }
}

pub fn get_char_by_index(text: String, i: usize) -> String {
    text.chars().nth(i).unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use crate::rtda::heap::method_descriptor::MethodDescriptor;

    #[test]
    fn should_get_desc_from_void_no_params() {
        let mut descriptor = MethodDescriptor::new(String::from("()V"));
        descriptor.parse();
        assert_eq!(0, descriptor.parameter_types.len());
        let option = descriptor.return_type;
        assert_eq!("V", option.str);
    }

    #[test]
    fn should_get_desc_from_void_string() {
        let string = String::from("(Ljava/lang/String;)V");
        let mut descriptor = MethodDescriptor::new(string);
        descriptor.parse();
        assert_eq!(1, descriptor.parameter_types.len());
        assert_eq!("Ljava/lang/String;", descriptor.parameter_types[0].str);
        let option = descriptor.return_type;
        assert_eq!("V", option.str);
    }
}
