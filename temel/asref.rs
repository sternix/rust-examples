fn work_on_bytes(slice: &[u8]) {}

fn work_on_bytes_asref<T: AsRef<[u8]>>(input: T) {
    let slice = input.as_ref();
}

fn main() {
    let vec = Vec::new();
    work_on_bytes(&vec);

    let arr = [0; 10];
    work_on_bytes(&arr);

    let slice = &[1, 2, 3];
    work_on_bytes(slice); // Note lack of &, since it doesn't need coercing

    // However, instead of explicitly requiring a slice, the function can be made to accept any type that can be used as a slice:
    work_on_bytes_asref(vec);
    work_on_bytes_asref(arr);
    work_on_bytes_asref(slice);
    work_on_bytes_asref("strings work too!");
}
