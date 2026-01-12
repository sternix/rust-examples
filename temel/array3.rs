/*
bir tipten, sabit uzunlukta olabilirler
hafızada tüm elemanlar yanyana bulunurlar
tanımlanırken atanmalıdırlar = [....]
indeks ile erişim çok hızlı
*/

fn main() {
    /*
    buradaki tipler yazılmayabilir
        let _a: [i32;4] = [1,2,3,4];
        let b: [&str;4] = ["this","is","a","test"];
        let _c: [[i32;2];2] = [[1,2],[3,4]];
        let d: [[i32;3];3] = [[1,2,3],[4,5,6],[7,8,9]];
        let e: [[u32;3];3] = [[1,2,3],[4,5,6],[7,8,9]];
    */

    let _a = [1, 2, 3, 4];
    let b = ["this", "is", "a", "test"];
    let _c = [[1, 2], [3, 4]];
    let d = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let e = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

    for s in &b {
        println!("{}", s);
    }

    for &i in &d {
        for j in &i {
            print!("{} ", j);
        }
        println!("");
    }

    for i in 0..e.len() {
        for j in 0..e[i].len() {
            print!("{} ", e[i][j]);
        }
        println!("");
    }
}
