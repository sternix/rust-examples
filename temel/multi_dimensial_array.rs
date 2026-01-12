fn main() {
    const M: usize = 2;
    const N: usize = 4;

    let grid = [[0 as u8; N]; M];
    //  yada type annotation ile
    //  let grid: [[u8; N]; M] = [[0; 4]; 2];

    for (_i, row) in grid.iter().enumerate() {
        for (_j, col) in row.iter().enumerate() {
            print!("{}", col);
        }
        println!()
    }

    // yada flat_map ile
    /*
        for el in grid.iter().flat_map(|r| r.iter()) {
            println!("{}", el);
        }
    */
}
