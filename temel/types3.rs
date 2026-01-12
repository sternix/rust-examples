fn main() {
    let imax = std::i32::MAX;
    println!("i32 Max: {}", imax);
    let x = imax + 1;
    println!("{}", x);
}

/*
Debug build'de panicliyor
Release build'de (rust -O max.rs)
i32 Max: 2147483647
-2147483648
*/
