/*

destructuring yaparken,
elemanlara bir şekilde erişebiliyoruz,
örnekte olduğu gibi point'in x ve y'sine erişebiliyoruz,
fakat bir değişken olarak point'e erişmek istersek

let DEĞİŞKEN @ TİP = değer
ile tipi bir değişkene atayabiliyoruz

*/

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

fn test() -> Point {
    Point { x: 3.14, y: 1.23 }
}

fn main() {
    let Point { x, y } = test();
    println!("{x}-{y}");

    let pt @ Point { x, y } = test();
    println!("{x}-{y}, {:?}", pt);
}
