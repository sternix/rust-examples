// https://twitter.com/Mastapegs/status/1508242428377157640?cxt=HHwWkMC-qeyfre4pAAAA

// A "Strong Number" is the number that the sum of the factorial of its digits is equal to number itself.

/*

145 = 1 + (1 * 2 * 3 * 4) + (1 * 2 * 3 * 4 * 5)
= 1 + 24 + 120
= 145

*/

fn factorial(number: u64) -> u64 {
    (1..=number).fold(1, |a, b| a * b)
}

fn is_strong(n: u64) -> bool {
    n == n
        .to_string()
        .chars()
        .map(|ch| factorial(ch.to_string().parse::<u64>().unwrap()))
        .sum()
}

fn main() {
    for i in 0..1_000_000 {
        if is_strong(i) {
            println!("{i} is strong");
        }
    }
}
