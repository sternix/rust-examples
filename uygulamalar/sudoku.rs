use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

// https://www.geeksforgeeks.org/sudoku-backtracking-7/
// C'den rust'a

// N is the size of the 2D matrix   N*N
const N: usize = 9;

/* A utility function to print grid */
fn print(arr: &[[usize; N]; N]) {
    for i in 0..N {
        for j in 0..N {
            print!("{} ", arr[i][j]);
        }
        println!();
    }
}

fn read_grid() -> [[usize; N]; N] {
    let file_name = env::args().nth(1).expect("Please supply a filename");
    let file = File::open(&file_name).expect("Can't open the file");
    let reader = BufReader::new(file);

    let mut grid = [[0usize; N]; N];

    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let sira: Vec<usize> = line
            .split(',')
            .map(|ch| ch.parse::<usize>().unwrap())
            .collect();
        if sira.len() != N {
            panic!("Every line must be {} chars", N);
        }

        for j in 0..N {
            grid[i][j] = sira[j];
        }
    }

    grid
}

// Checks whether it will be legal
// to assign num to the
// given row, col
fn is_safe(grid: &[[usize; N]; N], row: usize, col: usize, num: usize) -> bool {
    // Check if we find the same num
    // in the similar row , we return 0
    for x in 0..N {
        if grid[row][x] == num {
            return false;
        }
    }

    // Check if we find the same num in the
    // similar column , we return 0
    for x in 0..N {
        if grid[x][col] == num {
            return false;
        }
    }

    // Check if we find the same num in the
    // particular 3*3 matrix, we return false
    // 0,1,2,3 ve 6 değerlerini alıyorlar
    // 3,3 matrislerin ilk değerleri
    let start_row = row - row % 3;
    let start_col = col - col % 3;

    for i in 0..3 {
        for j in 0..3 {
            if grid[start_row + i][start_col + j] == num {
                return false;
            }
        }
    }

    true
}

/*
Takes a partially filled-in grid and attempts
to assign values to all unassigned locations in
such a way to meet the requirements for
Sudoku solution (non-duplication across rows, columns, and boxes)
 */
fn solve_suduko(mut grid: &mut [[usize; N]; N], mut row: usize, mut col: usize) -> bool {
    // Check if we have reached the 8th row and 9th column (0 indexed matrix) ,
    // we are returning true to avoid
    // further backtracking
    if row == N - 1 && col == N {
        return true;
    }

    //  Check if column value  becomes 9 , we move to next row and
    //  column start from 0
    if col == N {
        row += 1;
        col = 0;
    }

    // Check if the current position
    // of the grid already contains
    // value > 0, we iterate for next column
    if grid[row][col] > 0 {
        return solve_suduko(&mut grid, row, col + 1);
    }

    for num in 1..=N {
        // Check if it is safe to place
        // the num (1-9)  in the
        // given row ,col  ->we move to next column
        if is_safe(grid, row, col, num) {
            /* assigning the num in the
            current (row,col)
            position of the grid
            and assuming our assigned num
            in the position
            is correct */
            grid[row][col] = num;

            //  Checking for next possibility with next
            //  column
            if solve_suduko(grid, row, col + 1) {
                return true;
            }
        }

        // Removing the assigned num ,
        // since our assumption
        // was wrong , and we go for next
        // assumption with
        // diff num value
        grid[row][col] = 0;
    }

    false
}

fn main() {
    let mut grid = read_grid();

    if solve_suduko(&mut grid, 0, 0) {
        print(&grid);
    } else {
        println!("No solution exists");
    }
}

/*
dosya formatı bu şekilde olmalı
boş değer için 0 kullanılıyor

0,0,0,0,0,6,4,0,0
0,9,0,4,5,0,0,2,0
5,4,0,0,0,7,0,0,0
0,0,0,0,0,5,0,0,0
9,0,0,1,8,0,7,0,0
0,2,0,0,0,0,0,0,3
8,0,0,9,4,0,1,0,0
0,0,0,6,0,0,0,0,0
0,0,1,0,0,0,0,7,0
*/
