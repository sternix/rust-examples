fn main() {
    // Default
    assert_eq!(format!("{}", "bookends"), "bookends");

    // Minimum field width
    assert_eq!(format!("{:4}", "bookends"), "bookends");
    assert_eq!(format!("{:12}", "bookends"), "bookends    ");

    // Text length limit
    assert_eq!(format!("{:.4}", "bookends"), "book");
    assert_eq!(format!("{:.12}", "bookends"), "bookends");

    // Field width, length limit
    assert_eq!(format!("{:12.20}", "bookends"), "bookends    ");
    assert_eq!(format!("{:4.20}", "bookends"), "bookends");
    assert_eq!(format!("{:4.6}", "bookends"), "booken");
    assert_eq!(format!("{:6.4}", "bookends"), "book  ");

    // Aligned left, width
    assert_eq!(format!("{:<12}", "bookends"), "bookends    ");

    // Centered width
    assert_eq!(format!("{:^12}", "bookends"), "  bookends  ");

    // Aligned right, width
    assert_eq!(format!("{:>12}", "bookends"), "    bookends");

    // Pad with '=', centered width
    assert_eq!(format!("{:=^12}", "bookends"), "==bookends==");

    // Pad '*', aligned right, width, limit
    assert_eq!(format!("{:*>12.4}", "bookends"), "********book");

    assert_eq!(
        format!("{2:#06x},{1:b},{0:=>10}", "first", 10, 100),
        "0x0064,1010,=====first"
    );
}
