/*
 * @lc app=leetcode id=37 lang=rust
 *
 * [37] Sudoku Solver
 */

// @lc code=start
impl Solution {
    // Naive recursive backtracking
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut empties = vec![];

        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] == '.' {
                    empties.push((i, j));
                }
            }
        }

        fn search(board: &mut Vec<Vec<char>>, empties: &Vec<(usize, usize)>, n: usize) -> bool {
            if n == empties.len() {
                return true;
            }

            let (i, j) = empties[n];

            'next_digit: for c in "123456789".chars() {
                for k in 0..9 {
                    if board[i][k] == c { continue 'next_digit; }
                    if board[k][j] == c { continue 'next_digit; }
                    if board[i/3*3+k/3][j/3*3+k%3] == c { continue 'next_digit; }
                }
                board[i][j] = c;
                if search(board, empties, n+1) {
                    return true;
                }
            }

            board[i][j] = '.';
            return false;
        }

        search(board, &empties, 0);
    }
}
// @lc code=end

