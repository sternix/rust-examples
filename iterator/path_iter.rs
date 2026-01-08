use std::ffi::OsStr;
use std::path::Path;

fn main() {
    let path = Path::new("/home/user/files/test.iso");
    let mut iterator = path.iter();
    assert_eq!(iterator.next(), Some(OsStr::new("/")));
    assert_eq!(iterator.next(), Some(OsStr::new("home")));
    assert_eq!(iterator.next(), Some(OsStr::new("user")));
    assert_eq!(iterator.next(), Some(OsStr::new("files")));
    assert_eq!(iterator.next(), Some(OsStr::new("test.iso")));
    assert_eq!(iterator.next(), None);
}
