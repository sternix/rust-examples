fn main() {
    // C'de int grid[2][4] = {{1,2,3,4},{5,6,7,8}};
    // rust'ta [[u8; 4]; 2]
    let grid: [[u8; 4]; 2] = [[1, 2, 3, 4], [5, 6, 7, 8]];

    for i in 0..2 {
        for j in 0..4 {
            print!("{}-{}: {}, ", i, j, grid[i][j]);
        }
        println!();
    }
}
