fn main() {
    println!("{}", env!("HOME", "HOME set edilmemiş"));

    // ilginç olan bu derleme aşamasında error veriyor
    // println!("{}", env!("YOK","YOK set edilmemiş"));
}

// normal env metodları çalışma zamanında anlaşılır
// env! macrosu derleme aşamasında error veriyor
