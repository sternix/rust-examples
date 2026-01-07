fn main() {
    let a: [i32; 4] = [1, 2, 3, 4];
    let b: &[i32] = &a;
    let c = &a;
    let d = &a[..];

    let e = &a[1..2];
    let f = &a[1..];
    let g = &a[1..4]; // 4 hariç
    let h = &a[..3]; // 3 hariç

    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("c: {:?}", c);
    println!("d: {:?}", d);
    println!("e: {:?}", e);
    println!("f: {:?}", f);
    println!("g: {:?}", g);
    println!("h: {:?}", h);
}
