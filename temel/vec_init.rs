/*
The easiest way to create a vector is to use the vec! macro:
eğer elimizde hazır değerler varsa
*/

fn main() {
    // Create an empty vector
    let mut _numbers: Vec<i32> = vec![];

    // Create a vector with given contents
    let _words = vec!["step", "on", "no", "pets"];

    // 1024 zeroed-out bytes
    let mut _buffer = vec![0u8; 1024];
}

/*
A vector has three fields: the length, the capacity, and a pointer to a heap allocation where the elements are stored.

The empty vector numbers initially has a capacity 0, No heap memory is allocated for it until the first element is added.

Like all collections, Vec implements std::iter::FromIterator, so you can create a vector from any iterator using the iterator's .collect() method.

let my_vec = my_set.into_iter().collect::<Vec<String>>();
*/
