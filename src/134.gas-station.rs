/*
 * @lc app=leetcode id=134 lang=rust
 *
 * [134] Gas Station
 */
impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut g = 0;
        let mut g_min = 0;
        let mut i_min = 0;

        for i in 0..gas.len() {
            if g_min > g {
                g_min = g;
                i_min = i as i32;
            }

            g += gas[i];
            g -= cost[i];
        }

        if g >= 0 {
            i_min
        } else {
            -1
        }
    }
}

