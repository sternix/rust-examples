pub mod outer {
    pub fn say_hello() {
        inner::helper();
    }

    pub mod inner {
        // ikiside çalışıyor
        pub fn helper() {
            //pub (in crate) fn helper() {

            // HATA VERİYOR - main içinde
            //pub (in super) fn helper() {
            println!("Hello!");
        }
    }
}

fn main() {
    crate::outer::say_hello();
    crate::outer::inner::helper();
}
