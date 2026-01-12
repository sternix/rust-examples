fn main() {
    let arr = [1, 2, 3, 4];

    // eski versiyonlarda burası hata veriyordu
    for i in arr {
        println!("{i}");
    }

    for i in arr.iter() {
        println!("{i}");
    }

    for i in &arr {
        println!("{i}");
    }
}
