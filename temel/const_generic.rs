// const generics

pub fn ascii_procedural<const CHARS: usize>(text: &str) -> u32 {
    assert_eq!(text.as_bytes().len(), CHARS);

    text.as_bytes()
        .iter()
        .map(|c| c - '0' as u8)
        .fold(0, |acc, c| acc * 10 + c as u32)
}

fn main() {
    let mystr = "123456";
    let num = ascii_procedural::<6>(mystr);

    println!("{} -> {}", mystr, num);
}
