// pub fn nth(&mut self, n: usize) -> Option<Self::Item>

fn main() {
    let a = [1, 2, 3, 4, 5];
    let mut a = a.iter();

    for i in 1..6 {
        assert_eq!(a.nth(0).unwrap(), &i);
    }

    let a = [1, 2, 3, 4, 5];
    let mut a = a.iter();

    for _ in 0..5 {
        println!("{}", a.nth(0).unwrap());
    }
}

// sürekli 0 parametresi ile çağırırsak ( .nth(0) ) en öndeki değeri verir
// stack'teki pop metodu gibi
