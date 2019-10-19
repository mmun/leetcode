/*
 * @lc app=leetcode id=23 lang=rust
 *
 * [23] Merge k Sorted Lists
 */

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
use std::collections::BinaryHeap;

impl Solution {
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = BinaryHeap::new();

        for i in 0..lists.len() {
            if let Some(ref node) = lists[i] {
                heap.push((-node.val, i));
            }
        }

        let mut list = None;
        let mut current = &mut list;

        while let Some((_, i)) = heap.pop() {
            *current = lists[i].take();
            current = &mut current.as_mut().unwrap().next;
            lists[i] = current.take();

            if let Some(ref node) = lists[i] {
                heap.push((-node.val, i));
            }
        }

        list
    }
}
// @lc code=end
