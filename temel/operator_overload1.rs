use std::ops::Add;

#[derive(Debug)]
struct Age {
    age: i32,
}

impl Add for Age {
    type Output = Age;

    fn add(self, other: Age) -> Age {
        Age {
            age: self.age + other.age,
        }
    }
}

fn main() {
    let age1 = Age { age: 12 };
    let age2 = Age { age: 24 };
    let sum = age1 + age2;
    println!("age1 + age2 = {:?}", sum);
}
