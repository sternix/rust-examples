// mutable multi dimensial array

fn main() {
    const M: usize = 2;
    const N: usize = 4;

    let mut grid: [[u8; N]; M] = [[0; 4]; 2];

    for (_i, row) in grid.iter_mut().enumerate() {
        for (_j, col) in row.iter_mut().enumerate() {
            *col = 1;
        }
    }

    for (_i, row) in grid.iter().enumerate() {
        for (_j, col) in row.iter().enumerate() {
            print!("{}", col);
        }
        println!()
    }

    // yada

    let mut grid: [[u8; N]; M] = [[0; 4]; 2];

    for el in grid.iter_mut().flat_map(|r| r.iter_mut()) {
        *el = 1;
    }

    for (_i, row) in grid.iter().enumerate() {
        for (_j, col) in row.iter().enumerate() {
            print!("{}", col);
        }
        println!()
    }

    // yada klasik haliyle

    let mut grid: [[u8; N]; M] = [[0; 4]; 2];

    for i in 0..M {
        for j in 0..N {
            grid[i][j] = 1;
        }
    }

    for i in 0..M {
        for j in 0..N {
            print!("{}", grid[i][j]);
        }
        println!();
    }
}
