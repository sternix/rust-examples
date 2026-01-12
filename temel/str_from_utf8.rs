fn main() {
    let good_utf8: Vec<u8> = vec![65, 66, 67];
    assert_eq!(String::from_utf8(good_utf8).ok(), Some("ABC".to_string()));

    let bad_utf8: Vec<u8> = vec![0x9f, 0xf0, 0xa6, 0x80];
    let result = String::from_utf8(bad_utf8);
    assert!(result.is_err());
    // Since String::from_utf8 failed, it didn't consume the original vector, and the error value hands it back to us unharmed.
    assert_eq!(
        result.unwrap_err().into_bytes(),
        vec![0x9f, 0xf0, 0xa6, 0x80]
    );
}
