// 0'dan 1000'e kadar olan sayılardan 3'ün ve 5'in katlarının toplamı

// https://twitter.com/Mastapegs/status/1507740497925844992/photo/1

fn main() {
    let mut sum = 0;

    // klasik yol
    for i in 0..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }

    println!("Sum: {sum}");

    // iterator yolu
    let sum: u32 = (0..1000)
        .into_iter()
        .filter(|n: &u32| n % 3 == 0 || n % 5 == 0)
        .sum();

    println!("Sum: {sum}");
}
