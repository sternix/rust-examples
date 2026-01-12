// https://github.com/holgertkey/100-days-of-code/blob/master/Round1/R1D11/roman-numerals/src/lib.rs

const MAP: [(u32, &str); 13] = [
    (1, "I"),
    (4, "IV"),
    (5, "V"),
    (9, "IX"),
    (10, "X"),
    (40, "XL"),
    (50, "L"),
    (90, "XC"),
    (100, "C"),
    (400, "CD"),
    (500, "D"),
    (900, "CM"),
    (1_000, "M"),
];

fn int_to_roman(num: u32) -> String {
    let mut n = num;
    MAP.iter()
        .rev()
        .fold(String::new(), |mut string, &(value, symbol)| {
            while n >= value {
                string += &symbol.to_string();
                n -= value;
            }
            string
        })
}

fn main() {
    println!("{}", int_to_roman(9));
    println!("{}", int_to_roman(58));
    println!("{}", int_to_roman(1994));
    println!("{}", int_to_roman(4000));
}
