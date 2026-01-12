// burada &[i32] ifadesi i32 slice'ı
fn sum(values: &[i32]) -> i32 {
    let mut sum = 0;
    for i in 0..values.len() {
        sum += values[i];
    }
    sum
}

fn main() {
    let arr = [10, 20, 30, 40];
    // dikkat edilirse dizinin adresini yolluyoruz
    // bu işlem dizinin slice'ı oluyor
    println!("Dizinin elemanları toplamı: {}", sum(&arr));
}
