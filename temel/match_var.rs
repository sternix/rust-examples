// eğer bir match olursa, değer @'in önündeki değişkene atanıyor

fn main() {
    let number = 10;

    match number {
        digit @ 0..=10 => println!("digit: {}", digit),
        n => println!("diğer: {}", n),
    }
}
