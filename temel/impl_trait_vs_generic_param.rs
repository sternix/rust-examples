// impl Trait ile generic arguman farkı

use std::fmt::Display;

fn print_trait<T: Display>(val: T) {
    println!("{val}");
}

fn print_impl_trait(val: impl Display) {
    println!("{val}");
}

fn main() {
    print_trait("TEST");
    print_impl_trait("TEST");

    print_trait::<i32>(28);

    // `impl Trait` cannot be explicitly specified as a generic argument
    // bu hata veriyor
    // print_impl_trait::<i32>(28);
}
