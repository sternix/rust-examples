fn main() {
    for arg in std::env::args() {
        println!("'{}'", arg);
    }

    // varsa program adı hariç tüm parametreleri string vectörü yapıyor
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() > 0 {
        println!("{:?}", args);
    }

    let first = std::env::args().nth(1).expect("Lütfen bir sayı giriniz!");
    let n = first.parse().expect("Lütfen bir sayı giriniz!");
    println!("girilen sayı {}", n);
}
