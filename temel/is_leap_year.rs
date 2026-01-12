// artık yıl

pub fn is_leap_year(year: u64) -> bool {
    (year % 4 == 0) && (year % 100 != 0 || year % 400 == 0)
}

fn main() {
    let years = [2024, 2025, 2026];
    for y in years {
        println!("{y} is leap: {} ", is_leap_year(y));
    }
}
