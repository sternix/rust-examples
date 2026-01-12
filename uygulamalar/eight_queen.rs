const N: usize = 8;

fn print_solution(board: &[[usize; N]; N]) {
    for i in 0..N {
        for j in 0..N {
            print!("{}", board[i][j]);
        }
        println!();
    }
}

fn is_safe(board: &[[usize; N]; N], row: usize, col: usize) -> bool {
    for i in 0..col {
        if board[row][i] == 1 {
            return false;
        }
    }

    for (i, j) in (0..row + 1).rev().zip((0..col + 1).rev()) {
        println!("F{}-{}", i, j);
        if board[i][j] == 1 {
            return false;
        }
    }

    for (i, j) in (row..N).zip((0..col + 1).rev()) {
        println!("S{}-{}", i, j);
        if board[i][j] == 1 {
            return false;
        }
    }

    true
}

fn solve_nq(mut board: &mut [[usize; N]; N], col: usize) -> bool {
    if col >= N {
        return true;
    }

    for i in 0..N {
        if is_safe(&board, i, col) {
            board[i][col] = 1;
            if solve_nq(&mut board, col + 1) {
                return true;
            }
            board[i][col] = 0;
        }
    }

    false
}

fn main() {
    let mut board = [[0usize; N]; N];

    if solve_nq(&mut board, 0) {
        print_solution(&board);
    } else {
        println!("Solution does not exist");
    }
}
