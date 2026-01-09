fn main() {
    let mut val = 0;

    println!("{:p}", val as *mut i32); // null pointer
    println!("{:p}", &mut val as *mut i32); // the address of the variable
}
