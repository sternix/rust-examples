fn main() {
    // geçerli değişken adlandırmaları
    let age = 31;
    let _age = 31;
    let age1 = 31; // valid variable
    let age_num = 31; // valid variable
    let first_name = "Jack";

    // bu hata vermiyor fakat iki değişken olarak görülüyor
    // format ile 's@lary' -> 's @ lary' olarak değiştiriliyor
    let s @ lary = 52352;

    /*
    geçersiz değişken adlandırmaları

    let 1age = 31;
    let first name = "Jack";
    let first-name = "Jack";

    */
}
