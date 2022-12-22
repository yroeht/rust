/* https://leetcode.com/problems/merge-two-sorted-lists/ 21 */

struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }

    fn prepend(self, val: i32) -> Self {
        ListNode {
            next: Some(Box::new(self)),
            val
        }
    }
}

impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, l) | (l, None) => l,
            (Some(l1), Some(l2)) => {
                let ListNode{ next: next1, val: val1 } = *l1;
                let ListNode{ next: next2, val: val2 } = *l2;
                if val1 < val2 {
                    Some(Box::new(ListNode{
                        next: Solution::merge_two_lists(
                                  next1,
                                  Some(Box::new(ListNode{ next: next2, val: val2 }))),
                        val: val1}))
                } else {
                    Some(Box::new(ListNode{
                        next: Solution::merge_two_lists(
                                  Some(Box::new(ListNode{ next: next1, val: val1 })),
                                  next2),
                        val: val2}))
                }
            }
        }
    }
}

fn main() {
    let l1 = Some(Box::new(ListNode::new(4).prepend(2).prepend(1)));
    let l2 = Some(Box::new(ListNode::new(4).prepend(3).prepend(1)));
    dbg!(Solution::merge_two_lists(l1, l2));
    let l1 = None;
    let l2 = None;
    dbg!(Solution::merge_two_lists(l1, l2));
    let l1 = None;
    let l2 = Some(Box::new(ListNode::new(0)));
    dbg!(Solution::merge_two_lists(l1, l2));
}
