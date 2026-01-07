/*
Run Length Encoding

Implement run-length encoding and decoding.

Run-length encoding (RLE) is a simple form of data compression, where runs (consecutive data elements) are replaced by just one data value and count.

For example we can represent the original 53 characters with only 13.

"WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWB"  ->  "12WB12W3B24WB"

RLE allows the original data to be perfectly reconstructed from the compressed data, which makes it a lossless data compression.

"AABCCCDEEEE"  ->  "2AB3CD4E"  ->  "AABCCCDEEEE"

For simplicity, you can assume that the unencoded string will only contain the letters A through Z (either lower or upper case) and whitespace. This way data to be encoded will never contain any numbers and numbers inside data to be decoded always represent the count for the following character.
*/

use std::iter;

fn encode(source: &str) -> String {
    let mut result = String::new();
    let mut chars = source.chars().peekable();
    let mut count = 0;

    while let Some(c) = chars.next() {
        count += 1;

        if chars.peek() != Some(&c) {
            if count > 1 {
                result.push_str(&count.to_string());
            }
            result.push_str(&c.to_string());
            count = 0;
        }
    }
    result
}

fn decode(source: &str) -> String {
    source
        .chars()
        .filter(|c| !c.is_numeric())
        .zip(
            source
                // buradan sadece rakamlar geliyor !c.is_numeric ile karakterler diskalifiye ediliyor
                .split(|c: char| !c.is_numeric())
                // Dönen &str tipli rakamlar usize'a çevriliyor
                .map(|n| n.parse::<usize>().unwrap_or(1)),
        )
        .flat_map(|(c, n)| iter::repeat(c).take(n))
        .collect()
}

fn main() {
    assert_eq!("XYZ", encode("XYZ"));
    assert_eq!("", decode(""));
    assert_eq!("AABBBCCCC", decode("2A3B4C"));
    assert_eq!(
        "WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWB",
        decode("12WB12W3B24WB")
    );
    assert_eq!("  hsqq qww  ", decode("2 hs2q q2w2 "));
    assert_eq!("zzz ZZ  zZ", decode(encode("zzz ZZ  zZ").as_str()));
}
