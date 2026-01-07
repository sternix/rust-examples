// https://twitter.com/joshaustintech/status/1566478700228386816/photo/1

fn value_of(numeral: &char) -> i32 {
    match numeral {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => 0,
    }
}

fn roman_to_int(s: &str) -> i32 {
    let mut sum: i32 = 0;
    let mut index: usize = 0;
    let mut previous_value: i32 = 0;

    for numeral in s.chars() {
        let value = value_of(&numeral);
        sum += value;
        if index > 0 {
            if previous_value < value {
                sum -= previous_value * 2;
            }
        }
        previous_value = value;
        index += 1;
    }
    sum
}

fn print_numeral(s: &str) {
    println!("{}", roman_to_int(s));
}

fn main() {
    print_numeral("IX");
    print_numeral("LVIII");
    print_numeral("MCMXCIV");
    print_numeral("MMMM");
}
