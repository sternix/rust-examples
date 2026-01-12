// bir sayının palindrome olup olmadığı

fn num_is_palindrome(num: u64) -> bool {
    let num_string = num.to_string();
    let half = num_string.len() / 2;

    num_string.bytes().take(half).eq(num_string.bytes().rev().take(half))
}

// karakter eşitliği değil byte eşitliğine bakılıyor
fn is_palindrome(s: &str) -> bool {
    let half = s.len() / 2;
    s.bytes().take(half).eq(s.bytes().rev().take(half))
}

fn main() {
    let s = "amanaplanacanalpanama";
    if is_palindrome(s) {
        println!("palindrome");
    } else {
        println!("palindrome değil");
    }

    if num_is_palindrome(123454321) {
        println!("palindrome");
    } else {
        println!("palindrome değil");
    }
}
