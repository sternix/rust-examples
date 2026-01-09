fn main() {
    let mut x = 0;
    loop {
        if x > 3 {
            break;
        }
        println!("{x}");
        x += 1;
    }

    let mut x = 0;
    while x <= 3 {
        println!("{x}");
        x += 1;
    }
}
