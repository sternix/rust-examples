/*
Note: Method calls have a higher precedence than unary prefix operators,
so be careful when applying methods to negated values
*/

fn main() {
    println!("{}", -4_i32.abs());
    println!("{}", (-4_i32).abs());
}

// -4
// 4
