enum E {
    // Unit variant
    A,

    // Tuple variant
    B(i32),

    // Empty Struct variant
    C {},

    // Struct variant
    D { _x: i32 },
}

// Bu çeşit kullanıma
// "custom discriminant values" deniliyor
// struct veya tuple içeren bir enum'la discriminant değerler
// bir arada kullanılamıyor
// Hata Mesajı:
// custom discriminant values are not allowed in enums with tuple or struct variants
enum E2 {
    A,
    B = 100,
    C,
    D = 28,
    E,
    F = -35,
    G,
}

fn main() {
    let _e = E::A;
    print_e(_e);
    let _e = E::B(1234);
    print_e(_e);
    let _e = E::C {};
    print_e(_e);
    let _e = E::D { _x: 123 };
    print_e(_e);
    let _x = 1234;
    let _e = E::D { _x };
    print_e(_e);

    // Hata
    //assert_eq!(E::A as u32, 0);

    assert_eq!(E2::A as u32, 0);
    assert_eq!(E2::B as u32, 100);
    assert_eq!(E2::C as u32, 101);
    assert_eq!(E2::D as u32, 28);
    assert_eq!(E2::E as u32, 29);
    assert_eq!(E2::F as i32, -35);
    assert_eq!(E2::G as i32, -34);
}

fn print_e(e: E) {
    match e {
        E::A => println!("E::A"),
        E::B(n) => println!("E::B({})", n),
        E::C {} => println!("E::C{{}}"),

        // D'nin _x alanına başka bir alias tanımlayarak
        E::D { _x: d } => println!("E::D{{x:{}}}", d),
        // Yada _x'in değerini kullanarak
        // E::D{_x} => println!("E::D{{x:{}}}",_x),
    }
}
