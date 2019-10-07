/*
 * @lc app=leetcode id=63 lang=rust
 *
 * [63] Unique Paths II
 */

// @lc code=start
impl Solution {
    pub fn unique_paths_with_obstacles(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());

        let mut dp = vec![vec![0; n+1]; m+1];
        dp[0][1] = 1;

        for i in 1..=m {
            for j in 1..=n {
                if grid[i-1][j-1] == 0 {
                    dp[i][j] = dp[i-1][j] + dp[i][j-1];
                }
            }
        }

        dp[m][n]
    }
}
// @lc code=end

