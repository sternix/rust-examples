/*
to_digit(10) -> 10'luk sistem
to_digit(16) -> 16'luk sistem'e çeviriyor
*/

fn main() {
    let x = "134";
    println!(
        "{}",
        x.chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>()
    );
}
