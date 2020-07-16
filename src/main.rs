struct C;

struct B<'b> {
    c: &'b C,
}

struct A<'a> {
    b: B<'a>,
    c: &'a C, // now this is a reference too
}

impl<'a> A<'a> {
    fn new(c: &'a C) -> A<'a> {
        A {c: c, b: B{c: c}}
    }
}

fn main() {
    let c1 = C;
    let _ = A {
        c: &c1,
        b: B { c: &c1 },
    };
}
