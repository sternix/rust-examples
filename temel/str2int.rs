// inttostr
// strtoint

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // String to int
    let s1 = String::from("42");
    let n1 = s1.parse::<u64>()?;
    assert_eq!(n1, 42);

    // or
    let n2: u64 = s1.parse()?;
    let n3: i32 = s1.parse()?;
    assert_eq!(n2, 42);
    assert_eq!(n3, 42);

    // int to String
    let s2 = format!("{n2}");
    assert_eq!(s2, "42");

    Ok(())
}
