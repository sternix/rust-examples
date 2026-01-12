use std::collections::HashMap;

fn main() {
    let mut nufus = HashMap::new();
    nufus.insert("Ankara", 6_000_000);
    nufus.insert("İstanbul", 12_000_000);
    nufus.insert("Giresun", 100_000);
    nufus.insert("İzmir", 4_000_000);
    // DİKKAT:
    // burada n > 1_000_000 verilse Ankara, İstanbul yada İzmir'den birisi dönebilir
    // hangisinin olacağının garantisi yok,
    // fakat n > 6_000_000'a uyan bir tane İstanbul var,
    // find bulursa ilk true dönen değeri yada None döndürüyor
    assert_eq!(
        nufus.iter().find(|&(_s, &n)| n > 6_000_000),
        Some((&"İstanbul", &12_000_000))
    );
}
