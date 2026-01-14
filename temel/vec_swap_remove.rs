// vec'den bir elemanı almak, alınan yerine varsa son elemanı koyacak

/*

vec.swap_remove(index);
vec.remove()'dan farkı sıra önemli değil,
çıkan eleman yerine son elemanı koyuyor

*/

fn print_vec(v: &Vec<i32>) {
    for i in v {
        print!("{i}");
    }
    println!();
}

fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    print_vec(&v);
    let _ = take(&mut v, 2);
    print_vec(&v);
}

fn take<T>(vec: &mut Vec<T>, index: usize) -> Option<T> {
    if vec.get(index).is_none() {
        None
    } else {
        Some(vec.swap_remove(index))
    }
}
