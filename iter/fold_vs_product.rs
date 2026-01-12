fn main() {
    fn factorial(n: u32) -> u32 {
        (1..=n).product()
    }
    assert_eq!(factorial(0), 1);
    assert_eq!(factorial(1), 1);
    assert_eq!(factorial(5), 120);

    fn fact(n: u32) -> u32 {
        (1..n + 1).fold(1, |sum, value| sum * value)
    }

    assert_eq!(fact(0), 1);
    assert_eq!(fact(1), 1);
    assert_eq!(fact(5), 120);
}
