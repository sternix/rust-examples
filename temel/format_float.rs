fn main() {
    // Default
    assert_eq!(format!("{}", 1234.5678), "1234.5678");

    // Precision - Dikkat: .56'yı .57'ye yuvarladı
    assert_eq!(format!("{:.2}", 1234.5678), "1234.57");
    assert_eq!(format!("{:.6}", 1234.5678), "1234.567800");

    // Minimum field width
    assert_eq!(format!("{:12}", 1234.5678), "   1234.5678");

    // Minimum, precision- Dikkat: .56'yı .57'ye yuvarladı
    assert_eq!(format!("{:12.2}", 1234.5678), "     1234.57");
    assert_eq!(format!("{:12.6}", 1234.5678), " 1234.567800");

    // Leading zeros, minimum, precision
    assert_eq!(format!("{:012.6}", 1234.5678), "01234.567800");

    // Scientific
    assert_eq!(format!("{:e}", 1234.5678), "1.2345678e3");

    // Scientific, precision
    assert_eq!(format!("{:.3e}", 1234.5678), "1.235e3");

    // Scientific, minimum, precision
    assert_eq!(format!("{:12.3e}", 1234.5678), "     1.235e3");
    assert_eq!(format!("{:12.3E}", 1234.5678), "     1.235E3");
}
