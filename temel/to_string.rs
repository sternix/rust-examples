fn main() {
    let s = str::to_string("test");
    println!("{s}");

    let s = "test".to_string();
    println!("{s}");

    let s = ToString::to_string("test");
    println!("{s}");

    <str as ToString>::to_string("test");
    println!("{s}");
}

// full qualified method names
// dördüde aynı
