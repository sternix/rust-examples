fn main() {
    let mut a = [1, 2, 3, 4];

    // tipi &mut [i32;4];
    // e'nin tipi &mut i32
    for e in &mut a {
        *e = *e * 2;
    }

    // tipi &[i32;4];
    // e'nin tipi &i32
    for e in &a {
        println!("{}", e);
    }

    let mut v = vec![1, 2, 3, 4];

    // tipi &mut Vec<i32>
    // e'nin tipi &mut i32
    for e in &mut v {
        *e = *e * 3;
    }

    // tipi &Vec<T>
    // e'nin tipi &i32
    for e in &v {
        println!("{}", e);
    }

    // burada v move ediliyor
    // bundan sonra kullanılamıyor
    // tipi Vec<i32>
    // e'nin tipi i32
    for e in v {
        println!("{}", e);
    }

    // HATA
    //println!("len of v: {}", v.len());
}
