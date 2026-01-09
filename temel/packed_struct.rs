use std::mem::size_of;

/*

#[repr(C)] declares that you require interoperability with the C language.
Rust won’t rearrange the content of your structure.

#[repr(packed)]  tells  Rust  not  to  waste  space  on  your  structure.  This  can
carry  a  small  performance  penalty  but  guarantees  that  structures  are
exactly the right size.

*/

struct VeryImportantMessage {
    _message_type: u8,
    _destination: u16,
}

#[repr(C, packed)]
struct ReallyThreeBytes {
    a: u8,
    b: u16,
}

fn main() {
    println!(
        "VeryImportantMessage occupies {} bytes.",
        size_of::<VeryImportantMessage>()
    );

    println!(
        "ReallyThreeBytes occupies {} bytes.",
        size_of::<ReallyThreeBytes>()
    );
}
