// chain ile iki iterator'ü birleştirip tek iterator yapabiliriz

fn main() {
    let a1 = [1, 2, 3];
    let a2 = [4, 5, 6];

    for i in a1.iter().chain(a2.iter()) {
        println!("{i}");
    }
}
