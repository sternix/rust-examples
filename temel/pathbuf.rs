use std::path::PathBuf;

fn main() {
    let path = {
        let mut path = PathBuf::new();
        path.push("some");
        path.push("path");
        path.push("with");
        path.push("a");
        path.push("file.txt");
        path
    };

    assert_eq!(path, PathBuf::from("some/path/with/a/file.txt"));
}
