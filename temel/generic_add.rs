// generic örnek: hem int hem float'u toplayabiliyoruz

fn main() {
    let sum_int_value = addition_of_values(3, 5);
    println!("Addition of Integer values : {:?}", sum_int_value);

    let sum_float_value = addition_of_values(3.1, 5.5);
    println!("Addition of Float values : {:?}", sum_float_value);
}

pub fn addition_of_values<T: PartialOrd + std::ops::Add<Output = T>>(num1: T, num2: T) -> T {
    num1 + num2
}
