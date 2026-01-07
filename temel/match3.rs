use std::env;

fn main() {
    let arg = env::args().nth(1).expect("Lütfen bir sayı giriniz");
    let n: u32 = arg.parse().expect("Lütfen bir sayı giriniz");

    let size = match n {
        0...3 => "Small",
        4...6 => "Medium",
        _ => "Large",
    };

    println!("{}", size);
}

/*

Rust match statements can also match on ranges.
note that these ranges have three dots and are inclusive

match text n {
    0...3 => "Small",
    4...6 => "Medium",
    _ => "Large",
}

eğer n'nin değeri 3 olsa sonuç "Small" olur
[0,1,2,3] => Small
[4,5,6] => Medium
geriye kalanlar Large oluyor

*/
