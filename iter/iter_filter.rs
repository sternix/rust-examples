fn main() {
    let iter = (0..10).filter(|x| x % 2 == 0);
    for i in iter {
        println!("{i}");
    }
}

/*
0
2
4
6
8
*/
