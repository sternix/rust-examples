#[macro_export]
macro_rules! psum {
    ($($num:expr),*) => {
        {
            let mut sum = 0;
            $(
                sum += $num;
            )*
            sum
        }
    };
}

fn main() {
    let p = psum!(1, 2, 3, 4, 5, 6, 7);
    println!("{}", p);
}
