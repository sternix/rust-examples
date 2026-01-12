use std::collections::BTreeSet;

fn main() {
    let mut favorites = BTreeSet::new();
    favorites.insert("Test1".to_string());
    favorites.insert("Test2".to_string());

    // dikkat edilirse Option<&String> tipinde
    let mut it = favorites.iter();
    assert_eq!(it.next(), Some(&"Test1".to_string()));
    assert_eq!(it.next(), Some(&"Test2".to_string()));
    assert_eq!(it.next(), None);

    // dikkat edilirse Option<&String> tipinde
    // dikkat edilirse (&favorites) ile
    let mut it = (&favorites).into_iter();
    assert_eq!(it.next(), Some(&"Test1".to_string()));
    assert_eq!(it.next(), Some(&"Test2".to_string()));
    assert_eq!(it.next(), None);

    // eğer referans olmasaydı move edilecekti
    for f in &favorites {
        println!("{}", f);
    }

    // dikkat edilirse Option<String> tipinde
    let mut it = favorites.into_iter();
    assert_eq!(it.next(), Some("Test1".to_string()));
    assert_eq!(it.next(), Some("Test2".to_string()));
    assert_eq!(it.next(), None);

    // burada favorites'i bir daha kullanamayız move edildi
}
