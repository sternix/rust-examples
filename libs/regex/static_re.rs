
/*

[dependencies]
lazy_static = "1.5"
regex = "1"

*/

use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::Cow;

fn reformat_dates(before: &str) -> Cow<'_, str> {
    lazy_static! {
        static ref ISO8601_DATE_REGEX: Regex =
            Regex::new(r"(?<y>\d{4})-(?<m>\d{2})-(?<d>\d{2})").unwrap();
    }
    ISO8601_DATE_REGEX.replace_all(before, "$m/$d/$y") // <- XXX
}

fn main() {
    let before = "2012-03-14, 2013-01-15 and 2014-07-05";
    let after = reformat_dates(before);
    assert_eq!(after, "03/14/2012, 01/15/2013 and 07/05/2014");
}