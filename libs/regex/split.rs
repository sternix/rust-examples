// split ile ayırma

use regex::Regex;

fn main() {
    let re = Regex::new(r"\s+").unwrap();
    let splits = re.split("Hello  world!     How");
    assert_eq!(splits.collect::<Vec<&str>>(), &["Hello", "world!", "How"]);
}
