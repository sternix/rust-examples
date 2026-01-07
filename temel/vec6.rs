fn main() {
    let mut v1 = vec![1, 10, 5, 1, 2, 11, 2, 40];
    v1.sort();

    // sort işlemi gerekli
    // ardışık olanlardan birini siliyor
    // verilen vectör'de işlem yapıyor, mevcut elemanlardan
    // iki yada daha fazla olandan bir tane bırakıyor,
    // eğer orjinal vectör'deki elemanlara ihtiyaç varsa clone()
    // ile yedeğini almalıyız.
    v1.dedup();
    assert_eq!(v1, &[1, 2, 5, 10, 11, 40]);
}
