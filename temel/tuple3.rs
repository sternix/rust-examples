fn main() {
    let tup: (i32, f64, u8) = (6500, 45.7, 28);
    println!("0 = {}, 1 = {}, 2 = {}", tup.0, tup.1, tup.2);

    let (x, y, z) = tup;
    println!("x = {}, y = {}, z = {}", x, y, z);

    let tup = (500, 6.4, 1);
    println!("0 = {}, 1 = {}, 2 = {}", tup.0, tup.1, tup.2);

    let (x, y, z) = tup;
    println!("x = {}, y = {}, z = {}", x, y, z);

    let (_, y, _) = tup;
    println!("y = {}", y);
}
