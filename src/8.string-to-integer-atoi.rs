/*
 * @lc app=leetcode id=8 lang=rust
 *
 * [8] String to Integer (atoi)
 */

// @lc code=start
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();

        let mut i = 0;
        let mut sign: i64 = 1;
        let mut num: i64 = 0;

        while i < n && s[i] == b' ' {
            i += 1;
        }

        if i < n && s[i] == b'+' {
            i += 1;
        } else if i < n && s[i] == b'-' {
            sign = -1;
            i += 1;
        }

        while i < n {
            if s[i] >= b'0' && s[i] <= b'9' {
                num = 10*num + (s[i]-b'0') as i64;
                if sign == 1 && num > i32::max_value() as i64 {
                    num = i32::max_value() as i64;
                    break;
                }
                if sign == -1 && num > -(i32::min_value() as i64) {
                    num = -(i32::min_value() as i64);
                    break;
                }
            } else {
                break;
            }
            i += 1;
        }

        (sign * num) as i32
    }
}
// @lc code=end

