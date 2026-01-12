fn main() {
    // split returns an iterator.
    let strs = "bananas,apples,pear".split(",");
    for s in strs {
        println!("{s}");
    }

    // And can be "collected" in a Vec with the Iterator::collect method.
    let strings: Vec<&str> = "bananas,apples,pear".split(",").collect(); // ["bananas", "apples", "pear"]
    dbg!(strings);
}
