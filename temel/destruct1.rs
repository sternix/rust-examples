struct StTuple(i32, &'static str);

impl StTuple {
    fn print(&self) {
        println!("{}-{}", self.0, self.1);
    }
}

fn main() {
    let st = StTuple(1, "denememe");
    st.print();

    // unpack, destruct
    let StTuple(a, b) = st;
    println!("a: {}", a);
    println!("b: {}", b);

    let StTuple(_, b) = st;
    println!("b: {}", b);
}
