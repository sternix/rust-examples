fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for element in a.iter().rev() {
        println!("the value is: {}", element);
    }

    for i in 0..10 {
        println!("{}", i);
    }

    for i in (0..10).rev() {
        println!("{}", i);
    }
}
