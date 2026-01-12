/*
eğer sum değişkenlerine tip vermezsek

consider giving `sum` an explicit type
hatası alıyoruz
*/

fn main() {
    let sum: i32 = (0..5).sum();
    println!("sum was {}", sum);

    let sum: i64 = [10, 20, 30, 40].iter().sum();
    println!("sum was {}", sum);
}
