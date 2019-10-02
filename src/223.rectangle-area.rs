/*
 * @lc app=leetcode id=223 lang=rust
 *
 * [223] Rectangle Area
 */
use std::cmp::{max, min};

impl Solution {
    pub fn compute_area(l1: i32, b1: i32, r1: i32, t1: i32, l2: i32, b2: i32, r2: i32, t2: i32) -> i32 {
        let w = if min(r1, r2) > max(l1, l2) { min(r1, r2) - max(l1, l2) } else { 0 };
        let h = if min(t1, t2) > max(b1, b2) { min(t1, t2) - max(b1, b2) } else { 0 };

        (r1-l1)*(t1-b1) - w*h + (r2-l2)*(t2-b2)
    }
}

