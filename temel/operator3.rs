// Logical Operators

fn main() {
    let a = true;
    let b = false;

    let c = !a; //false
    let d = a && b; //false
    let e = a || b; //true

    // On integer types,! inverts the individual bits in the two's complement representation of the value.

    let a = !-2; //1
    let b = !-1; //0
    let c = !0; //-1
    let d = !1; //-2
}
