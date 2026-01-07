fn main() {
    for i in (0..21).filter(|x| x % 2 == 0 && x % 3 == 0) {
        print!("{i} ");
    }
    println!();
}

// 0 6 12 18
