fn make_vec() -> Vec<i32> {
    let mut vec = Vec::new();
    for i in 0..=5 {
        vec.push(i);
    }
    vec
}

// buradaki vec parametresi
// &Vec<i32> ile aynı
// biri slice diğeri referans

//fn print_vec(vec: &Vec<i32>) {
fn print_vec(vec: &[i32]) {
    for i in vec {
        print!("{} ", i)
    }
    println!("");
}

fn main() {
    let vec = make_vec();

    // hepsi slice
    print_vec(&vec);
    print_vec(&vec[..]);
    print_vec(&vec[0..6]);
    print_vec(&vec[0..vec.len()]);
    print_vec(&vec[..vec.len() - 1]);
    print_vec(&vec[1..3]);
    print_vec(&vec[..3]);
    print_vec(&vec[3..]);

    for i in vec {
        println!("{}", i)
    }

    // burada vec kullanılamıyor
    // ^^^ value used here after move
}
