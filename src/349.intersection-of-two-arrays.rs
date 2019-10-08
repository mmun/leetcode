/*
 * @lc app=leetcode id=349 lang=rust
 *
 * [349] Intersection of Two Arrays
 */

// @lc code=start
use std::collections::HashSet;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let set1 = nums1.into_iter().collect::<HashSet<_>>();
        let set2 = nums2.into_iter().collect::<HashSet<_>>();

        HashSet::intersection(&set1, &set2).cloned().collect::<Vec<_>>()
    }
}
// @lc code=end
