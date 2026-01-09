// https://twitter.com/Mastapegs/status/1507810193517723648/photo/1

fn generate_prime_numbers_up_to(number: u32) -> Vec<u32> {
    let mut potential_primes: Vec<u32> = (2..=number).into_iter().collect();
    let mut primes: Vec<u32> = Vec::new();

    while potential_primes.len() > 0 {
        let a_prime = potential_primes.remove(0);
        primes.push(a_prime);
        potential_primes = potential_primes
            .into_iter()
            .filter(|number| number % a_prime != 0)
            .collect();
    }

    primes
}

fn main() {
    assert_eq!(vec![2, 3, 5, 7, 11], generate_prime_numbers_up_to(11));
}
