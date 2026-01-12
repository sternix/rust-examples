fn main() {
    let name = "Jack";
    let age = 31;

    println!("Name: {}, Age={}", name, age);
    println!("Name: {0}, Age={1}", name, age);
    println!("Name: {1}, Age={0}", age, name);
    println!("Name: {name}, Age={age}");
}
