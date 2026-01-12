struct I32Range {
    start: i32,
    end: i32,
}

impl Iterator for I32Range {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        if self.start >= self.end {
            return None;
        }

        let result = Some(self.start);
        self.start += 1;
        result
    }
}

fn main() {
    for i in (I32Range { start: 0, end: 10 }) {
        println!("{}", i);
    }
}
