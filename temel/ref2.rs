fn by_ref(x: &i32) -> i32 {
    *x + 1
}

fn main() {
    let i = 10;
    let res1 = by_ref(&i);
    let res2 = by_ref(&42);
    println!("{} - {}", res1, res2);
}

/*
ilginç olan C'de 42'nin adresini alamıyoruz.

#include <stdio.h>

int by_ref(int *x) {
        return *x + 1;
}

int main() {
        int i = 10;
        // cannot take the address of an rvalue of type 'int'
        // printf("%d\n", by_ref(&42));
        printf("%d\n", by_ref(&i));
        return 0;
}
*/
