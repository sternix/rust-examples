fn main() {
    let v = 0;

    let x = &v as *const _ as usize;
    println!("{}", x);
    let p = x as *const i64;
    println!("{:?}", p);
}
