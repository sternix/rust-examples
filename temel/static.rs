static G: i32 = 10;
static mut SM: i32 = 11;

fn main() {
    // mut değil güvenli, unsafe gerekmiyor
    println!("G: {}", G);

    // mut olduğunda unsafe gerekiyor
    println!("SM: {}", unsafe { SM });

    unsafe { SM += 9 }

    println!("SM: {}", unsafe { SM });

    for i in 1..100 {
        assert_eq!(i, static_local())
    }
}

fn static_local() -> i32 {
    static mut SL: i32 = 0;
    unsafe {
        SL += 1;
        SL
    }
}
