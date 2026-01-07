fn main() {
    for i in 0..10i64 {
        // yada 10u8 vs
        println!("{}", i);
    }

    // range'ler dinamik olabilir
    fn x(n: u32) {
        for i in 0..n {
            println!("{}", i)
        }
    }
    x(5);
}
