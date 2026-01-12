/*
dizi sıralama
sıralanacak dizinin elemanı std::cmp::Ord'u impl etmeli
yoksa
the trait bound `X: std::cmp::Ord` is not satisfied
*/

fn main() {
    let mut a = [3, 5, 2, 4, 1];
    a.sort();
    assert_eq!(a, [1, 2, 3, 4, 5]);
}
