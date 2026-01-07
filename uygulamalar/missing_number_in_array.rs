// https://practice.geeksforgeeks.org/problems/missing-number-in-array1416/1

fn main() {
    let arr = [1, 2, 3, 4, 6, 7, 8];

    let n = arr.len() + 1;
    let olmasi_gereken = (n * (n + 1)) / 2;
    let mevcut: usize = arr.iter().sum();
    let eksik = olmasi_gereken - mevcut;

    println!("{}", eksik);
}
