/*
 * @lc app=leetcode id=219 lang=rust
 *
 * [219] Contains Duplicate II
 */

// @lc code=start
use std::collections::HashSet;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;

        let mut set = HashSet::new();

        for i in 0..nums.len() {
            if !set.insert(nums[i]) {
                return true;
            }
            if i >= k {
                set.remove(&nums[i - k]);
            }
        }

        return false;        
    }
}
// @lc code=end

