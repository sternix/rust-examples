// https://practice.geeksforgeeks.org/problems/plus-one/1/

fn main() {
    let arr = [1, 2, 3];

    let mut sum = 0;
    for (i, n) in arr.iter().rev().enumerate() {
        let b = u32::pow(10, i as u32);
        sum += n * b;
    }
    sum += 1;

    println!("sum: {}", sum);

    // ikinci yol
    // string birleştirme
    let mut sum = 0;
    for (i, n) in arr.iter().rev().enumerate() {
        let mut s = String::from("1");
        s.push_str(&"0".repeat(i));
        let b: u32 = s.parse().unwrap();
        sum += n * b;
    }
    sum += 1;

    println!("sum: {}", sum);
}
