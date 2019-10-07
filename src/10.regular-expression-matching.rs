/*
 * @lc app=leetcode id=10 lang=rust
 *
 * [10] Regular Expression Matching
 */

// @lc code=start
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let (s, p) = (s.as_bytes(), p.as_bytes());
        let (m, n) = (s.len(), p.len());

        // dp[i][j] = s[..i] matches p[..j]
        let mut dp = vec![vec![false; n+1]; m+1];

        dp[0][0] = true;
        for j in 1..=n {
            if p[j-1] == b'*' {
                dp[0][j] = dp[0][j-2];
            }
        }
    
        for i in 1..=m {
            for j in 1..=n {
                if p[j-1] == b'.' || p[j-1] == s[i-1] {
                    dp[i][j] = dp[i-1][j-1];
                }
                if p[j-1] == b'*' {
                    if p[j-2] == b'.' || p[j-2] == s[i-1] {
                        dp[i][j] = dp[i][j-2] || dp[i-1][j];
                    } else {
                        dp[i][j] = dp[i][j-2];
                    }
                }
            }
        }

        dp[m][n]
    }
}
// @lc code=end
