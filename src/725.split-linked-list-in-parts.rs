/*
 * @lc app=leetcode id=725 lang=rust
 *
 * [725] Split Linked List in Parts
 */
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
impl Solution {
    pub fn split_list_to_parts(root: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
        let k = k as usize;
        let mut n = 0;

        let mut current = &root;
        while let Some(node) = current {
            n += 1;
            current = &node.next;
        }

        let mut parts = Vec::new();
        let mut head = root;

        for i in 0..k {
            let mut tail = &mut head;

            for j in 0..(n / k + (i < n % k)) {
                tail = &mut tail.as_mut().unwrap().next;
            }

            let new_head = tail.take();
            parts.push(head);
            head = new_head;
        }

        parts
    }
}

