/*
in situation like this, Rust simply creates an anonymous variable to hold the expression's value
and makes the reference point to  that. The lifetime of this anonymous variable depends on what you do with the reference
*/

fn factorial(n: usize) -> usize {
    (1..n + 1).fold(1, |a, b| a * b)
}

fn main() {
    let r = &factorial(6);
    // r == 720
    assert_eq!(r + 1009, 1729);
    assert_eq!(r + &1009, 1729);
}
