// https://stackoverflow.com/questions/37215739/what-does-it-mean-to-instantiate-a-rust-generic-with-an-underscore-e-g-err/37215830#37215830

fn main() {
    let bar = [1, 2, 3];
    let foos = bar
        .iter()
        .map(|x| format!("{}", x))
        .collect::<Vec<String>>(); // <- turbofish

    println!("{:?}", foos);

    // diğer bir yol

    let bar = [1, 2, 3];
    let foos = bar.iter().map(|x| format!("{}", x)).collect::<Vec<_>>();

    println!("{:?}", foos);

    // diğer bir yol

    let bar = [1, 2, 3];
    let foos: Vec<_> = bar // <-- specify a type and use '_' to make the compiler, compiler tipini tahmin etsin
        .iter() 
        .map(|x| format!("{}", x))
        .collect(); // <-- no more turbofish

    println!("{:?}", foos);
}
