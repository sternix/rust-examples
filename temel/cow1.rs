/*

name: &str
yerine
cow

*/

use std::borrow::Cow;

#[derive(Debug)]
struct City<'a> {
    name: Cow<'a, str>,
    date_founded: u32,
}

fn main() {
    let city = City {
        name: Cow::from("Giresun"),
        date_founded: 1000,
    };

    println!("{:?}", city);
}
