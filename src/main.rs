struct C;

struct B<'b> {
    c: &'b C,
}

struct A<'a> {
    b: B<'a>,
    c: &'a C, // now this is a reference too
}

fn main() {
    let c1 = C;
    let _ = A {
        c: &c1,
        b: B { c: &c1 },
    };
}
