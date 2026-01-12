/*

bir değişkenin tipini
std::any::type_name_of_val(&var);

ile alabiliriz

*/

fn get_iter() -> impl Iterator<Item = i32> {
    vec![1, 2, 3].into_iter()
}

fn main() {
    let iter = get_iter();
    let iter_name = std::any::type_name_of_val(&iter);
    let sum: i32 = iter.sum();
    println!("The sum of the `{iter_name}` is {sum}.");
}
