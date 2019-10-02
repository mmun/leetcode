/*
 * @lc app=leetcode id=794 lang=rust
 *
 * [794] Valid Tic-Tac-Toe State
 */
impl Solution {
    pub fn valid_tic_tac_toe(board: Vec<String>) -> bool {
        let b = |i: usize, j: usize| { board[i].as_bytes()[j] };
        let win = |x: u8| {
            false
                || (b(0, 0) == x && b(0, 1) == x && b(0, 2) == x)
                || (b(1, 0) == x && b(1, 1) == x && b(1, 2) == x)
                || (b(2, 0) == x && b(2, 1) == x && b(2, 2) == x)
                || (b(0, 0) == x && b(1, 0) == x && b(2, 0) == x)
                || (b(0, 1) == x && b(1, 1) == x && b(2, 1) == x)
                || (b(0, 2) == x && b(1, 2) == x && b(2, 2) == x)
                || (b(0, 0) == x && b(1, 1) == x && b(2, 2) == x)
                || (b(0, 2) == x && b(1, 1) == x && b(2, 0) == x)
        };
        
        let x_win = win(b'X');
        let o_win = win(b'O');
        let mut x_count = 0;
        let mut o_count = 0;
        
        for i in 0..3 {
            for j in 0..3 {
                if b(i, j) == b'X' { x_count += 1; }
                if b(i, j) == b'O' { o_count += 1; }
            }
        }

        if x_win { return !o_win && x_count == o_count + 1; }
        if o_win { return !x_win && x_count == o_count; }
        x_count == o_count || x_count == o_count + 1
    }
}

