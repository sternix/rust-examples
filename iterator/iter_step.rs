// pub fn step_by(self, step: usize) -> StepBy<Self>
// birer birer değil verdiğimiz değer kadar ilerliyor

fn main() {
    let a = [1, 2, 3, 4, 5, 6, 7];

    for i in a.iter().step_by(2) {
        println!("{i}");
    }
}

/*
1
3
5
7
*/
