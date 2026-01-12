fn main() {
    let width = 10;
    println!("left aligned:  |{:/<width$}|", "foo");
    println!("centered:      |{:/^width$}|", "foo");
    println!("right aligned: |{:/>width$}|", "foo");
}

/*

sonuç:
left aligned:  |foo///////|
centered:      |///foo////|
right aligned: |///////foo|

*/
