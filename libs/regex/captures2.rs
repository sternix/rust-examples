// eşleşme bulunmama kontrolü

use regex::Regex;

fn main() {
    let re = Regex::new(r"(?<word>\w+) world").unwrap();
    if let Some(caps) = re.captures("hello world") {
        assert_eq!("hello", &caps["word"]);
    } else {
        println!("Eşleşme bulunamadı");
    }
}
