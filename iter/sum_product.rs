/*
std::iter::Sum
std::iter::Product

trait'lerini kullanıyorlar
*/

fn sum(n: u64) -> u64 {
    (1..=n).sum()
}

// aslında factöriyel'in yaptığı işi yapıyor
fn product(n: u64) -> u64 {
    (1..=n).product()
}

fn main() {
    assert_eq!(sum(20), 210);
    assert_eq!(product(20), 2432902008176640000);
}
