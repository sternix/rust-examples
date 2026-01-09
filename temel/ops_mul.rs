fn multiply<T: std::ops::Mul<Output = T>>(t1: T, t2: T) -> T {
    t1 * t2
}

fn main() {
    assert_eq!(6, multiply(2u8, 3u8));
    assert_eq!(5.0, multiply(1.0, 5.0));
}
