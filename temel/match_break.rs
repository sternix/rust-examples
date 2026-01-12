// https://twitter.com/AstraKernel/status/1630477548835733505

fn main() {
    let mut i = 0;

    loop {
        match i {
            2 => break println!("Yes"),
            _ => println!("No"),
        }
        i += 1;
    }
}

/*
burada loop'a () değeri dönüyor,
çıktısı
No
No
Yes
*/
