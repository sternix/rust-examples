fn main() {
    let args: Vec<String> = std::env::args().collect();
    // ilk parametre programın yani exe'nin adı
    println!("Program Adı: {}", args[0]);

    // eğer varsa diğerleri kullanıcıların girdikleri parametreler
    if args.len() > 1 {
        println!("args: {:?}", args);
        // eğer i32 tipinde bir parametre istiyorsak
        let _i: i32 = args[1]
            .parse()
            .expect("Lütfen int tipinde bir parametre giriniz!");
    } else {
        println!("Lütfen bir parametre giriniz!");
    }
}
