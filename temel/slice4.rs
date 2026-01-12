// &[T] => bir tipin slice'ı
// slice hem array'lardan hemde vector'den olabilir
fn prints(s: &[i32]) {
    for i in s {
        print!("{} ", i);
    }
    println!("");
}

fn main() {
    let arr = [1, 2, 3, 4, 5];
    let s = &arr;
    prints(s);

    let a = &arr[..];
    prints(a);

    let b = &arr[0..];
    prints(b);

    let c = &arr[..5]; // 6'da panic'liyor
    prints(c);

    let d = &arr[3..5];
    prints(d);
}
