/*
 * @lc app=leetcode id=220 lang=rust
 *
 * [220] Contains Duplicate III
 */

// @lc code=start
use std::collections::BTreeSet;

impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        let k = k as usize;

        if t < 0 {
            return false;
        }

        let mut set = BTreeSet::new();

        for i in 0..nums.len() {
            let l = nums[i].saturating_sub(t);
            let r = nums[i].saturating_add(t);

            for _ in set.range(l..=r) {
                return true;
            }

            set.insert(nums[i]);

            if i >= k {
                set.remove(&(nums[i - k]));
            }
        }

        return false; 
    }
}
// @lc code=end

