/*
<'_>
yada sadece
'_

fn make_tester<'a>(answer: &'a str) -> impl Fn(&str) -> bool + 'a {
    move |challenge| {
        challenge == answer
    }
}

*/

fn make_tester(answer: &str) -> impl Fn(&str) -> bool + '_ {
    move |challenge| challenge == answer
}

fn main() {
    let test = make_tester("hunter2");
    println!("{}", test("*******"));
    println!("{}", test("hunter2"));
}
