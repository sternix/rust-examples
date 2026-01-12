use std::io::Write;

fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {
    let _ = out.write(b"A");
    out.flush()
}

fn main() {
    // eskiden let mut v = vec![1_u8, 2_u8];
    let mut v = vec![1, 2];
    let _ = say_hello(&mut v);
    for s in &v {
        println!("{s}");
    }
}

/*

1
2
65

*/
