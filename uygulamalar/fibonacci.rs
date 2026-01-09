// https://twitter.com/Mastapegs/status/1507758915123040269/photo/1
// fibonacci değeri 4_000_000'dan küçük olanların içinden çift olanların toplamı

fn fib(n: u32) -> u32 {
    match n {
        1 => 1,
        2 => 2,
        _ => fib(n - 1) + fib(n - 2),
    }
}

fn main() {
    let mut even_sums = 0;

    for i in 1.. {
        let fib = fib(i);
        println!("{fib}");
        if fib > 4_000_000 {
            break;
        }

        if fib % 2 == 0 {
            even_sums += fib;
        }
    }

    println!("Sum: {even_sums}");
}
