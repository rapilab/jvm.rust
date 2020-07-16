pub fn dot_to_slash(text: String) -> String {
    text.replace(".", "/")
}

pub fn slash_to_dot(text: String) -> String {
    text.replace("/", ".")
}

#[cfg(test)]
mod tests {
    use crate::rtda::path_conv::{dot_to_slash, slash_to_dot};

    #[test]
    fn should_con_dot_to_slash() {
        let string = dot_to_slash(String::from("a.b.c"));
        assert_eq!("a/b/c", string);
    }

    #[test]
    fn should_con_slash_to_dot() {
        let string = slash_to_dot(String::from("a/b/c"));
        assert_eq!("a.b.c", string);
    }
}
