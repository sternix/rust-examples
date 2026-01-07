// Convert temperatures between Fahrenheit and Celsius.

fn main() {
    println!("{:.2}", f2c(86));
    println!("{:.2}", c2f(30));
}

fn f2c(f: i32) -> f64 {
    ((f - 32) as f64) / 1.8
}

fn c2f(c: i32) -> f64 {
    (c as f64 * 1.8) + 32 as f64
}
