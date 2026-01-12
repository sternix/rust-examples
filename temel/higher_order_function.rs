// fonksiyon döndüren fonksiyon

fn main() {
    let step_value = &10;
    let step_function = higer_order_fn_return(step_value);
    println!("the stepped value is {}", step_function(50));
}

fn higer_order_fn_return<'a>(step_value: &'a i32) -> Box<dyn Fn(i32) -> i32 + 'a> {
    Box::new(move |x: i32| x + step_value)
}
