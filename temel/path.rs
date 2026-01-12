use std::ffi::OsStr;
use std::path::Path;

fn main() {
    let _home_dir = Path::new("/home/test");

    assert_eq!(
        Path::new("/home/test/doc.txt").parent(),
        Some(Path::new("/home/test"))
    );

    assert_eq!(
        Path::new("/home/test/doc.txt").file_name(),
        Some(OsStr::new("doc.txt"))
    );

    // absolute mutlak path /fff/dgfdg gibi
    // c:\Program Files\ gib
    assert_eq!(Path::new("/home/test/doc.txt").is_absolute(), true);

    // relative path ../.. vs gibi
    assert_eq!(Path::new("test/doc.txt").is_relative(), true);

    let path1 = Path::new("/usr/share/dict");
    assert_eq!(path1.join("words"), Path::new("/usr/share/dict/words"));

    // parametre absolute path ise parametreyi gönderiyor
    assert_eq!(path1.join("/home/test"), Path::new("/home/test"));
}

// These methods works on strings in memory.
// Paths also have some methods that query the filesystem: .exists(), .is_file(), .is_dir(), .read_dir(), .canonicalize(), and so on.
