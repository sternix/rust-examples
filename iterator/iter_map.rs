fn main() {
    let a = [1, 2, 3, 4];

    for i in a.iter().map(|x| x * x) {
        println!("{}", i);
    }
}

// burada for döngüsünde map ile oluşturulmuş veri yapısını kullanıyoruz (consume)
