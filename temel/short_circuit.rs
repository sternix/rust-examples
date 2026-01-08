// || ve && operator'lerine short circuit yani kısa devre operatörleri deniliyor

fn get_false() -> bool {
    println!("FALSE");
    false
}

fn get_true() -> bool {
    println!("TRUE");
    true
}

fn main() {
    // eğer birinci ifade yanlış ise ikici ifade çalıştırılmıyor
    if get_false() && get_true() {
        println!("Burası ÇALIŞMIYOR!!!!");
    }
    //FALSE

    // eğer birinci ifade doğru ise ikici ifade çalıştırılmıyor
    if get_true() || get_false() {
        println!("Burası ÇALIŞIYOR");
    }
    // TRUE
}
