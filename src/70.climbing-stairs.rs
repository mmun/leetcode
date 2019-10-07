/*
 * @lc app=leetcode id=70 lang=rust
 *
 * [70] Climbing Stairs
 */

// @lc code=start
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut s = (1, 1);
        for _ in 1..(n as usize) {
            s = (s.1, s.0 + s.1);
        }
        return s.1;
    }
}
// @lc code=end
