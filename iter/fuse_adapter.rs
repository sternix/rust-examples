struct Flaky(bool);

impl Iterator for Flaky {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 {
            self.0 = false;
            Some("totally the last item")
        } else {
            self.0 = true;
            None
        }
    }
}

fn main() {
    let mut flaky = Flaky(true);
    assert_eq!(flaky.next(), Some("totally the last item"));
    assert_eq!(flaky.next(), None);
    assert_eq!(flaky.next(), Some("totally the last item"));
    assert_eq!(flaky.next(), None);
    assert_eq!(flaky.next(), Some("totally the last item"));
    // ...
    //normalde iterator None döndürdükten sonra bitmesi gerekiyor,
    // fakat örnek'teki gibi bir Some bir None şeklinde sonsuza kadar devam eder
    // eğer bir defa None döndükten sonra, daha Some dönmemesini istersek
    // fuse() metodunu kullanıyoruz

    let mut not_flaky = Flaky(true).fuse();
    assert_eq!(not_flaky.next(), Some("totally the last item"));
    assert_eq!(not_flaky.next(), None);
    assert_eq!(not_flaky.next(), None);
    assert_eq!(not_flaky.next(), None);
}

// fuse adapter takes any iterator and turns into one that will definitely continue to return None once it has done so the first time.
