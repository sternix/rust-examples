fn main() {
    let s1 = "hello".to_string();
    println!("{:p}", &s1);
    println!("{:p}", &s1 as &str);
}
