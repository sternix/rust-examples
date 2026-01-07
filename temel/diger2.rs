fn main() {
    // println!("{}", 2.0.sqrt()); => HATA
    println!("{}", 2.0_f64.sqrt());
    println!("{}", 2.0_f32.sqrt());
    println!("{}", f64::sqrt(2.0));
    println!("{}", f32::sqrt(2.0));
}
