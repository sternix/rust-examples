fn main() {
    let mut sum = 0;
    for i in 0..5 {
        sum += i;
    }

    println!("sum: {}", sum);

    // eğer sonucun float olmasını istersek
    let mut sumf = 0.0;
    for i in 0..5 {
        sumf += i as f32;
    }

    println!("sum: {}", sumf);
}
