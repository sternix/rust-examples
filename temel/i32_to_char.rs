fn main() {
    let my_number = 65;

    //  only `u8` can be cast as `char`, not `i32`
    //  println!("{}", my_number as char);
    println!("{}", my_number as u8 as char);
}

// my_number varsayılan olarak i32
// my_number'ı direk char'a çeviremiyoruz önce u8 sonra char
