/*
 * @lc app=leetcode id=3 lang=rust
 *
 * [3] Longest Substring Without Repeating Characters
 */
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut counts = [0; 256];
        let mut bad = 0;
        let mut i = 0;
        let mut j = 0;
        let s = s.as_bytes();

        while j < s.len() {
            counts[s[j] as usize] += 1;
            if counts[s[j] as usize] > 1 {
                bad += 1;
            }
            j += 1;

            if bad > 0 {
                if counts[s[i] as usize] > 1 {
                    bad -= 1;
                }
                counts[s[i] as usize] -= 1;
                i += 1;
            }
        }

        return (j - i) as i32;
    }
}

