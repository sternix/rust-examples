struct S {
    x: i32,
}

fn main() {
    let _a = 5;

    // ikisi aynı işlevi görüyor
    let _r = &_a;
    let ref _r2 = &_a;

    let mut s = S { x: 6 };
    let x = &mut s.x;
    *x = 28;

    // daha karışık
    let x = &mut (S { x: 6 }).x;
    *x = 28;

    // aynı işlevi ref ile daha temiz yapıyoruz
    let S { ref mut x } = S { x: 6 };
    *x = 28;
}
