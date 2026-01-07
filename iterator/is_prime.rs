// bir sayı asal mı?

fn is_prime(n: u64) -> bool {
    (2..n).all(|divisor| n % divisor != 0)
}

// bu daha hızlı
// stop testing at the square root of n
fn is_prime_2(n: u64) -> bool {
    (2..n)
        .take_while(|divisor| divisor * divisor <= n)
        .all(|divisor| n % divisor != 0)
}

fn main() {
    for i in 0..100 {
        if is_prime(i) {
            println!("{i} Asal");
        };
    }
}
