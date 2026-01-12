fn main() {
    let a1 = [0; 10];
    let a2 = [true; 5];
    let a3 = ['*'; 6];
    let _one_kb = [0u8; 1024]; // 1 kb buffer tüm elemanları sıfır

    println!("{:?}", a1);
    println!("{:?}", a2);
    println!("{:?}", a3);
}
