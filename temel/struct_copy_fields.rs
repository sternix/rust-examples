struct St {
    f1: i32,
    f2: char,
    f3: &'static str,
}

impl St {
    fn print(&self) {
        println!("{}-{}-{}", self.f1, self.f2, self.f3);
    }
}

fn main() {
    let st = St {
        f1: 1,
        f2: 'x',
        f3: "xyz",
    };
    st.print();

    // tüm alanları st'den al
    let st = St { ..st };
    st.print();

    // f1 hariç tüm alanları st'den al
    let st = St { f1: 2, ..st };
    st.print();

    // f1 ve f2 hariç tüm alanları st'den al
    let st = St {
        f1: 3,
        f2: 'y',
        ..st
    };
    st.print();
}
