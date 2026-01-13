use regex::Regex;

fn main() {
    let text = "Hello 123 world! 42 is 10";
    let re = Regex::new(r"\d+").unwrap(); // \d means digit
    for mat in re.find_iter(text) {
        println!("{}", &text[mat.start()..mat.end()]);
    }
}

// Prints:
// 123
// 42
// 10
