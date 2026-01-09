fn some_work() -> Option<i32> {
    Some(28)
}

fn main() {
    let Some(_) = some_work() else {
        println!("None geldi");
        // return olmazsa hata veriyor
        return;
    };
}

// aynı şekilde Result'dada çalışıyor