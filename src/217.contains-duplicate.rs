/*
 * @lc app=leetcode id=217 lang=rust
 *
 * [217] Contains Duplicate
 */

// @lc code=start
use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = HashSet::new();

        for i in 0..nums.len() {
            if !set.insert(nums[i]) {
                return true;
            }
        }

        return false;
    }
}
// @lc code=end
