/*

Rust's From trait is a general-purpose trait for converting between types. For any two types TypeA and TypeB,

impl From<TypeA> for TypeB

indicates that an instance of TypeB is guaranteed to be constructible from an instance of TypeA. An implementation of From looks like this:

*/

struct TypeA {
    a: u32,
}

struct TypeB {
    _b: u32,
}

impl From<TypeA> for TypeB {
    fn from(src: TypeA) -> Self {
        TypeB { _b: src.a }
    }
}

fn main() {
    let a = TypeA { a: 28 };
    let _b: TypeB = a.into();
}
