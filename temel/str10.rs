fn main() {
    assert_eq!("ONE".to_lowercase(), "one");
    assert_eq!("peanut".contains("nut"), true);
    assert_eq!("<html>".replace("<", "&lt;"), "&lt;html>");
    assert_eq!(" \ttext\n".trim(), "text");
}
