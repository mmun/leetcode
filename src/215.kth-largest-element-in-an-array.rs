/*
 * @lc app=leetcode id=215 lang=rust
 *
 * [215] Kth Largest Element in an Array
 */
fn bubble_down<T: Ord>(nums: &mut Vec<T>, k: usize) {
    let n = nums.len();
    let mut i = k;

    while 2*i + 1 < n {
        let mut c = i;
        if 2*i + 1 < n && nums[c] < nums[2*i + 1] { c = 2*i + 1; }
        if 2*i + 2 < n && nums[c] < nums[2*i + 2] { c = 2*i + 2; }
        if c == i { break; }
        nums.swap(i, c);
        i = c;
    }
}


impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        
        for i in (0..=(n-1)/2).rev() {
            bubble_down(&mut nums, i);
        }

        for i in 1..k {
            nums[0] = nums.pop().unwrap();
            bubble_down(&mut nums, 0);
        }

        nums[0]
    }
}
