// Unit Struct - Zero Sized
struct S1;

// Tuple Struct
struct S2(i32);

// birden çok değerli Tuple Struct
struct S3(String, i32);

// Normal struct
struct S4 {
    _x: String,
    _y: i32,
}

// Normal struct ama elemanları yok
struct S5 {}

fn main() {
    let _s = S1;
    let _s = S2(123);
    let _s = S3("abc".into(), 123);
    let _s = S4 {
        _x: "klmn".into(),
        _y: 1234,
    };
    let _x = "xyz".to_string();
    let _y = 12345;

    // struct alanları ile local değişken isimleri aynı
    let _s = S4 { _x, _y };

    let _s = S5 {};
}
