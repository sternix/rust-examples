// panic!() makrosu panik için kullanılabilir. parametre olarak format stringi alabiliyor.

fn main() {
    let n = 2154;

    let is_even = |x: u64| x % 2 == 0;

    if is_even(n) {
        panic!("{} is even", n);
    }
}
