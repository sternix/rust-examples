fn main() {
    let text = "01234567891011121314151617181920";
    let (head, tail) = text.split_at(10);
    println!("head: {}", head);
    println!("tail: {}", tail);

    // ikisi de aynı işlevi görüyor

    let temp = text.split_at(10);
    let head = temp.0;
    let tail = temp.1;
    println!("head: {}", head);
    println!("tail: {}", tail);
}
