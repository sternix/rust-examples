struct GrayscaleMap {
    pixels: Vec<u8>,
    size: (usize, usize),
}

/*

new_map fonksiyonunun parametreleri struct'ın alanları ile aynı adlı olduğundan
aşağıdaki gibi kısa bir şekilde yazılabilir

 */

fn new_map(size: (usize, usize), pixels: Vec<u8>) -> GrayscaleMap {
    assert_eq!(pixels.len(), size.0 * size.1);
    // GrayscaleMap { pixels: pixels, size: size } // ile aynı
    GrayscaleMap { pixels, size }
}

fn main() {
    let width = 1024;
    let height = 576;

    // normal field bazında oluşturma
    let image = GrayscaleMap {
        pixels: vec![0; width * height],
        size: (width, height),
    };
}
