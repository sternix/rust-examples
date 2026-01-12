fn main() {
    let numbers = 0..;

    let five_numbers = numbers.take(5);

    for n in five_numbers {
        println!("{n}");
    }
}
