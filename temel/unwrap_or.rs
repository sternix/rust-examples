fn main() {
    let maybe_cake = Some("Chocolate cake");
    let not_cake = None;

    println!("{}", maybe_cake.unwrap());

    // bu panic'liyor
    // unwrap'tan farkı kendi mesajımızla paniklemesi
    //println!("{}", not_cake.expect("The cake is a lie"));

    // eğer None ise bizim verdiğimiz değeri döndürüyor
    println!("{}", not_cake.unwrap_or("Cheesecake"));

    // unwrap_or gibi fakat unwrap_or'da sabit bir değer verilirken
    // unwrap_or_else'de uygun bir değer yollayan bir fonksiyon veriliyor
    println!("{}", not_cake.unwrap_or_else(|| { "Pumpkin cake" }));

    // burada descruction yapılıyor
    match maybe_cake {
        Some(cake) => println!("{} was consumed", cake),
        None => println!("There was no cake"),
    }

    if let Some(cake) = maybe_cake {
        println!("{} was consumed", cake);
    }
}
