/*
 * @lc app=leetcode id=1006 lang=rust
 *
 * [1006] Clumsy Factorial
 */
impl Solution {
    pub fn clumsy(n: i32) -> i32 {
        let mut f = 0;
        let mut i = n;

        while i > 0 {
            let mut a = i;
            if i > 1 { a *= i-1; }
            if i > 2 { a /= i-2; }
            if i == n { f += a; } else { f -= a; }
            if i > 3 { f += i-3; }
            i -= 4;
        }

        f
    }
}

