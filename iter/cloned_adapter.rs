fn main() {
    let a = ['1', '2', '3', '~'];
    assert_eq!(a.iter().next(), Some(&'1'));

    // DİKKAT: &'1' yerine '1'
    assert_eq!(a.iter().cloned().next(), Some('1'));
}

/*
The cloned adapter takes an iterator that produces references and returns an iterator
that produces values cloned from those references. Naturally, the referent type must implement Clone
*/
