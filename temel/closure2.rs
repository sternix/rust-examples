fn ten_times<F>(f: F)
where
    F: Fn(i32),
{
    for i in 0..10 {
        f(i);
    }
}

fn prt(i: i32) {
    println!("hello, {}", i);
}

fn main() {
    ten_times(prt);

    ten_times(|j| println!("hello, {}", j));

    let word = "hello".to_owned();
    ten_times(move |j| println!("{}, {}", word, j));
}
