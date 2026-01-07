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

    /* panicliyor
    byte index 1 is not a char boundary; it is inside 'ü' (bytes 0..2) of `ü`'
    let x = "ü"; // ü bir byte'dan fazla yer işgal ediyor ü yerine a olsa sorun yok
    println!("{}", &x[0..1]);

    println!("{}", &x[0..2]); => çalışıyor ü
    */
}

/*
Hi! üğişıçö
len: 17
count: 11
Türkçe karakterler: üğişıçö
*/
