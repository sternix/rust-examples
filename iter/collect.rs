use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList};

/*
Bu tiplerin hepsi

std::iter::FromIterator trait'ini impl etmiş
*/

fn main() {
    let args: HashSet<String> = std::env::args().collect();
    let args: BTreeSet<String> = std::env::args().collect();
    let args: LinkedList<String> = std::env::args().collect();

    let args: HashMap<String, usize> = std::env::args().zip(0..).collect();
    let args: BTreeMap<String, usize> = std::env::args().zip(0..).collect();
}
