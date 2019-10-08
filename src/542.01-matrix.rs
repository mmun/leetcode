/*
 * @lc app=leetcode id=542 lang=rust
 *
 * [542] 01 Matrix
 */

// @lc code=start
use std::collections::VecDeque;

impl Solution {
    pub fn update_matrix(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (matrix.len(), matrix[0].len());

        let mut dist = vec![vec![-1; n]; m];
        let mut visited = vec![vec![false; n]; m];
        let mut queue = VecDeque::new();

        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == 0 {
                    queue.push_back((i, j));
                }
            }
        }

        let mut d = 0;

        while !queue.is_empty() {
            let len = queue.len();

            for _ in 0..len {
                let (i, j) = queue.pop_front().unwrap();

                if visited[i][j] { continue; }
                visited[i][j] = true;
                dist[i][j] = d;

                if i > 0 { queue.push_back((i-1, j)); }
                if j > 0 { queue.push_back((i, j-1)); }
                if i < m-1 { queue.push_back((i+1, j)); }
                if j < n-1 { queue.push_back((i, j+1)); }
            }

            d += 1;
        }

        dist
    }
}
// @lc code=end

