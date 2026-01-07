// değişken gölgeleme (Variable shadowing)

fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("x = {}", x); // sonuç 12
}
