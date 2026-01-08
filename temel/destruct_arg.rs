fn xyz(a: (i32, i32)) {
    println!("{} - {}", a.0, a.1);
}

fn xyz2((a, b): (i32, i32)) {
    println!("{} - {}", a, b);
}

fn main() {
    xyz((10, 20));
    xyz2((10, 20));
}
