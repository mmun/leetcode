/*
 * @lc app=leetcode id=841 lang=rust
 *
 * [841] Keys and Rooms
 */
impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut visited = vec![false; rooms.len()];
        let mut stack = vec![0];
        let mut count = 0;

        while let Some(i) = stack.pop() {
            if visited[i] { continue; }
            visited[i] = true;
            count += 1;

            for j in &rooms[i] {
                stack.push(*j as usize);
            }
        }

        count == rooms.len()
    }
}

