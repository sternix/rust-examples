fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 }; // dikkat edilirse burada ; var

    println!("The value of number is: {}", number);
}

/*

if'in tüm şartlarda aynı tip'te bir değer dönemesi gerekir
bu hata verir

fn main() {
    let condition = true;

    let number = if condition {
        5
    } else {
        "six"
    };

    println!("The value of number is: {}", number);
}
*/
