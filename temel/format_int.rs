fn main() {
    // Default
    assert_eq!(format!("{}", 1234), "1234");

    // Forced sign
    assert_eq!(format!("{:+}", 1234), "+1234");

    // Minimum field width
    assert_eq!(format!("{:12}", 1234), "        1234");

    // Sign, width
    assert_eq!(format!("{:+12}", 1234), "       +1234");

    // Leading zeros, width
    assert_eq!(format!("{:012}", 1234), "000000001234");

    // Sign, zeros, width
    assert_eq!(format!("{:+012}", 1234), "+00000001234");

    // Aligned left, width
    assert_eq!(format!("{:<12}", 1234), "1234        ");

    // Centered, width
    assert_eq!(format!("{:^12}", 1234), "    1234    ");

    // Aligned right, width
    assert_eq!(format!("{:>12}", 1234), "        1234");

    // Aligned left, sign, width
    assert_eq!(format!("{:<+12}", 1234), "+1234       ");

    // Centered, sign, width
    assert_eq!(format!("{:^+12}", 1234), "   +1234    ");

    // Aligned right, sign, width
    assert_eq!(format!("{:>+12}", 1234), "       +1234");

    // Padding with '=', centered, width
    assert_eq!(format!("{:=^12}", 1234), "====1234====");

    // Binary notation
    assert_eq!(format!("{:b}", 1234), "10011010010");

    // Width, octal notation
    assert_eq!(format!("{:12o}", 1234), "        2322");

    // Sign, width, hexadecimal notation
    assert_eq!(format!("{:+12x}", 1234), "        +4d2");

    // Sign, width, hex with capital digits
    assert_eq!(format!("{:+12X}", 1234), "        +4D2");

    // Sign, explicit radix prefix, width, hex
    assert_eq!(format!("{:+#12x}", 1234), "      +0x4d2");

    // Sign, radix,zeros, width, hex
    assert_eq!(format!("{:+#012x}", 1234), "+0x0000004d2");
    assert_eq!(format!("{:+#06x}", 1234), "+0x4d2");
}
