fn main() {
    let x = 5;

    // {}'nın içi bir expression'dır, bir değer üretir
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}
