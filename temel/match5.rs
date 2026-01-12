// eğer bir match'te bir şey yapmayacaksak {}, ile boş bir ifade yazıp geçeriz

fn main() {
    let i = 50;

    match i {
        0 => {}
        1 => println!("1"),
        _ => println!("diger"),
    }
}
