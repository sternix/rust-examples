fn main() {
    println!(
        "{:.3}us: relocated {} at {:#x} to {:#x}, {} bytes",
        0.84391, "object", 140737488346304_u64, 6299664_usize, 64
    );

    let s = format!("number of {}: {}", "elephants", 19);
    assert_eq!(s, "number of elephants: 19");

    let s = format!("from {1} to {0}", "the grave", "the cradle");
    assert_eq!(s, "from the cradle to the grave");

    let s = format!("v = {:?}", vec![0, 1, 2, 3]);
    assert_eq!(s, "v = [0, 1, 2, 3]");

    let s = format!("name = {:?}", "Nemo");
    assert_eq!(s, "name = \"Nemo\"");

    println!("name = {:?}", "Nemo");

    let s = format!("{:8.2} km/s", 11.186);
    assert_eq!(s, "   11.19 km/s"); // 3 tane boşluk var

    let s = format!("{:10} {:02x} {:02x}", "adc #42", 105, 42);

    assert_eq!(s, "adc #42    69 2a");

    // buradaki 1: 2: 0: parametre indeksi
    let s = format!("{1:02x} {2:02x}  {0}", "adc #42", 105, 42);
    assert_eq!(s, "69 2a  adc #42");

    let s = format!(
        "{lsb:02x} {msb:02x} {insn}",
        insn = "adc #42",
        lsb = 105,
        msb = 42
    );
    assert_eq!(s, "69 2a adc #42");

    assert_eq!(format!("{{a,c}} {{a,b,c}}"), "{a,c} {a,b,c}");
}
