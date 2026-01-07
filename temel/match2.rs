use std::env;

fn main() {
    let arg = env::args().nth(1).expect("Lütfen bir sayı giriniz");
    let n = arg.parse().expect("Lütfen bir sayı giriniz");
    // let n: i32..... ile aynı
    // eğer u32 istersek let n: u32 ....

    let text = match n {
        e if e < 0 => "Negative",
        0 => "Zero",
        1 => "One",
        2 => "Two",
        _ => "More",
    };

    println!("{}", text);
}

/*

let page_no = match page_no {
    n if n <= 0 => 1,
    n if n > total_no_of_pages => total_no_of_pages,
    _ => page_no,
};

*/
