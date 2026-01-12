// cargo ile derlenen projede çalışıyor

const EXAMPLES_DIR: &str = concat![env!("CARGO_MANIFEST_DIR"), "/examples"];

fn main() {
    println!("{}", EXAMPLES_DIR);
}
