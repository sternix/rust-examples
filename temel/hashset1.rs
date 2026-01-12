use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Viking {
    name: String,
    power: usize,
}

fn main() {
    let mut vikings = HashSet::new();

    vikings.insert(Viking {
        name: "X".into(),
        power: 1,
    });
    vikings.insert(Viking {
        name: "Y".into(),
        power: 2,
    });
    vikings.insert(Viking {
        name: "Z".into(),
        power: 3,
    });
    vikings.insert(Viking {
        name: "W".into(),
        power: 4,
    });

    for v in &vikings {
        println!("{v:?}");
    }
    if vikings.contains(&Viking {
        name: "Z".into(),
        power: 3,
    }) {
        println!("We got this viking in our HashSet");
    } else {
        println!("We aren't able to find corresponding result");
    }
}
