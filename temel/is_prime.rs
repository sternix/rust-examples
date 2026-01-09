fn is_prime(n: u64) -> bool {
    match n {
        0..=1 => false,
        _ => !(2..n).any(|d| n % d == 0),
    }
}

fn is_prime2(n: u64) -> bool {
    // 0,1 yanlış olarak true dönüyor
    (2..n)
        .take_while(|divisor| divisor * divisor <= n)
        .all(|divisor| n % divisor != 0)
}

fn main() {
    for i in 0..20 {
        println!("{}->{} ", i, is_prime(i));
        println!("{}->{} ", i, is_prime2(i));
    }
}
