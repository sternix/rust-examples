#[derive(Debug)]
struct S {
    _x: i32,
    _y: char,
    _z: String,
}

struct TS(i32, i32);

fn main() {
    let s = S {
        _x: 1,
        _y: 'Y',
        _z: "Z".into(),
    };
    println!("{:?}", s);

    let s = S { _x: 2, ..s };
    println!("{:?}", s);

    let _ts = TS(1, 2);

    // bu sadece tuple struct'larda kullanılabilir
    let _ts = TS { 0: 1, 1: 2 };
    // sıra önemli değil
    let _ts = TS { 1: 1, 0: 2 };
    // ..X ifadesine Base struct deniliyor
    let _ts = TS { .._ts };

    // HATA Base struct son eleman olmalı
    // let _ts = TS { .._ts, 0: 34};

    // !!! bu şekilde kullanamıyoruz,
    // Base struct için {} gerekli
    // let _ts = TS(.._ts);

    // istediğimiz değeri biz belirleyip,
    // diğer değerleri başka bir örnekten alabiliriz
    let _ts = TS { 0: 2, .._ts };
}
