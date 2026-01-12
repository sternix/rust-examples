fn main() {
    let multilingual = "Hi! üğişıçö";
    for ch in multilingual.chars() {
        print!("{}", ch);
    }
    println!("");
    println!("len: {}", multilingual.len());
    println!("count: {}", multilingual.chars().count());
    let maybe = multilingual.find('ü');
    if maybe.is_some() {
        let tr = &multilingual[maybe.unwrap()..];
        println!("Türkçe karakterler: {}", tr);
    }

    // yukarıdaki yapının match hali
    match multilingual.find('ü') {
        Some(idx) => {
            println!("Türkçe karakterler: {}", &multilingual[idx..]);
        }
        None => println!("ü karakteri bulunamadı!"),
    }

    // yada sadece Some sonuçları ile ilgileniliyorsa
    if let Some(idx) = multilingual.find('ü') {
        println!("Türkçe karakterler: {}", &multilingual[idx..]);
    }
}
