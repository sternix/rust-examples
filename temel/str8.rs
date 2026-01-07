fn main() {
    let method = b"GET";
    assert_eq!(method, &[b'G', b'E', b'T']);

    let path = r"C:\Program Files\Xyz";
    println!("{}", path);

    let free = r###"
    Burada istediğini yaz
\\ "
    "###;
    println!("{}", free);
}
