fn main() {
    let ints = [1, 2, 3, 4, 5];

    for s in ints.windows(2) {
        println!("window {:?}", s);
    }

    for s in ints.chunks(2) {
        println!("chunks {:?}", s);
    }
}

/*
window [1, 2]
window [2, 3]
window [3, 4]
window [4, 5]

chunks [1, 2]
chunks [3, 4]
*/
