// çalışan exe'nin full path'i

fn main() {
    println!("{}",std::env::current_exe().unwrap().to_str().unwrap());
}

// ...  /rust-examples/temel/./path_of_current_exe
