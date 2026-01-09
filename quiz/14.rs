// https://dtolnay.github.io/rust-quiz/14

// Scope Trait

trait Trait: Sized {
    fn is_reference(self) -> bool;
}

// impl<'a, T> Trait for &'a T {
impl<T> Trait for &T {
    fn is_reference(self) -> bool {
        true
    }
}

fn main() {
    match 0.is_reference() {
        true => print!("1"),
        false => print!("0"),
    }

    match '?'.is_reference() {
        true => print!("1"),
        false => {
            impl Trait for char {
                fn is_reference(self) -> bool {
                    false
                }
            }
            print!("0")
        }
    }
}

/*

Trait impls anywhere in a program are always in scope, so there is no significance to the impl Trait for char being written inside of a block of code. In particular, that impl is visible throughout the whole program, not just within the block containing the impl.
The call to 0.is_reference() observes that there is no implementation of Trait for an integer type that we could call directly. Method resolution inserts an auto-ref, effectively evaluating (&0).is_reference(). This time the call matches impl<'a, T> Trait for &'a T and prints 1.
The call to '?'.is_reference() instead finds impl Trait for char, printing 0.


ilginç olan kodun herhangi bir yerinde bir impl yapıldığında global scope'taki gibi davranıyor
char için match'teki false'de impl edilen is_reference uygulanıyor
sonuç 10

derlerken warning veriyor

`impl` definition, `impl` blocks should be written at the same level as their item
`Trait` is not local

*/
