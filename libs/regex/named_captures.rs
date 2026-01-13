/*

Named Capturing Groups

(?P<ADI>)
yenilerde P kullanılmıyor
(?<ADI>)

eğer capture bulunursa &caps["ADI"]

*/

use regex::Regex;

fn main() {
    let re = Regex::new(r"(?P<word>\w+) world").unwrap();
    let caps = re.captures("hello world").unwrap();

    assert_eq!("hello", &caps["word"]);
}
