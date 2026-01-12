// normalde fonksiyonun lifetime'lı hali
// fn smallest<'a>(v: &'a[i32]) -> &'a i32 {
fn smallest(v: &[i32]) -> &i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if *r < *s {
            s = r;
        }
    }
    s
}

fn main() {
    let s;
    /* burada parabola dizisinin bir elemanını ödünç alıyoruz, fakat blok'tan
       çıkınca parabola dizisi siliniyor s hala hayatta
    {
        let parabola = [9,4,1,0,1,4,9];
        s = smallest(&parabola); // ^^^^^^^^^ borrowed value does not live long enough
                     //`parabola` does not live long enough
    }
    - `parabola` dropped here while still borrowed
    */

    // bu şekilde sorun yok
    let parabola = [9, 4, 1, 0, 1, 4, 9];
    s = smallest(&parabola);

    assert_eq!(*s, 0);
}
