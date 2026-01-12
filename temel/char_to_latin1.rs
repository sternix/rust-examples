// latin1 ASCII'ye denk geliyor, 0xff yani 255 karakter

fn latin1_to_char(latin1: u8) -> char {
    latin1 as char
}

fn char_to_latin1(c: char) -> Option<u8> {
    if c as i32 <= 0xff {
        Some(c as u8)
    } else {
        None
    }
}

fn main() {
    assert_eq!(latin1_to_char(65), 'A');
    assert_eq!(char_to_latin1('A'), Some(65));

    let mut s = String::new();
    s += "XYZ";

    let mut s = String::with_capacity(50);
    s.push_str("XYZ");

    // burada "XYZ" slice oluyor
    let _s = "XYZ".to_string();

    let spacey = "man hat tan";
    let spaceless: String = spacey.chars().filter(|c| !c.is_whitespace()).collect();
    assert_eq!(spaceless, "manhattan");

    let mut s = String::with_capacity(100);
    assert_eq!(s.len(), 0);
    assert_eq!(s.capacity(), 100);

    s.push_str("hello");
    assert_eq!(s.len(), 5);
    assert_eq!(s.capacity(), 100);
    s.shrink_to_fit();
    assert_eq!(s.len(), 5);
    assert_eq!(s.capacity(), 5);

    s.reserve(60);
    assert_eq!(s.len(), 5);
    assert_eq!(s.capacity(), 65);

    s.clear();
    assert_eq!(s.len(), 0);
    assert_eq!(s.capacity(), 65);

    s.push_str("hello");
    s.truncate(3);
    assert_eq!(s, "hel");

    assert_eq!(s.pop().unwrap(), 'l');
    assert_eq!(s.pop().unwrap(), 'e');
    assert_eq!(s.pop().unwrap(), 'h');
    assert!(s.pop().is_none());

    s.push_str("hello");
    assert_eq!(s.remove(2), 'l');
    assert_eq!(s, "helo");

    let mut choco = "chocolate".to_string();
    assert_eq!(choco.drain(3..6).collect::<String>(), "col");
    assert_eq!(choco, "choate");

    let mut aaa = "abcdefgh".to_string();
    aaa.drain(4..);
    assert_eq!(aaa, "abcd");
}
