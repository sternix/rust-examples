// ikiside aynı işlevi görüyor

fn build_vec1() -> Vec<i16> {
    let mut v: Vec<i16> = Vec::<i16>::new();
    v.push(10i16);
    v.push(20_i16);
    v
}

fn build_vec2() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v
}

fn main() {
    assert_eq!(build_vec1(), build_vec2());
}
