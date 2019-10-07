/*
 * @lc app=leetcode id=64 lang=rust
 *
 * [64] Minimum Path Sum
 */

// @lc code=start
use std::cmp::min;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut best = vec![vec![0; n]; m];

        best[0][0] = grid[0][0];
        for i in 1..m { best[i][0] = best[i-1][0] + grid[i][0]; }
        for j in 1..n { best[0][j] = best[0][j-1] + grid[0][j]; }

        for i in 1..m {
            for j in 1..n {
                best[i][j] = min(best[i-1][j], best[i][j-1]) + grid[i][j];
            }
        }

        best[m-1][n-1]
    }
}
// @lc code=end
