/*
std::panic_catch_unwind() => catch stack unwinding,
allowing the thread to survive and continue running

Rust panic'ler Java'daki RuntimeException'a benziyor,
yani kontrollü

eğer bir panic meydana gelirse Rust hafızadaki tüm nesneleri drop etmeye çalışır
eğer bir sorun olmazsa tüm nesneler kontrollü olarak drop edilir,
eğer drop esnasında bir panic daha olursa Rust process'i abort eder.

eğer
rustc -C panic=abort panic_catch_unwind.rs
ile derlenirse ilk panic'te process'i abort eder ve işletim sistemi core dump oluşturur.
catch_unwind sadece unwind'leri yakalayabilir
abort işlemini yakalayamaz, son iki println çalışmaz program panic'te sonlanır.
*/

use std::panic;

fn main() {
    let n = 2;

    let is_even = |x: u64| x % 2 == 0;

    let res = panic::catch_unwind(|| {
        if is_even(n) {
            panic!("{} is even", n);
        }
    });

    if res.is_err() {
        println!("PANIC yakalandı");
    }

    println!("Program çalışmaya devam ediyor");
}
