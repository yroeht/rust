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
}

struct Solution;

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>)
        -> Option<Box<ListNode>> {
        match head {
            None => None,
            Some(b) => {
                let ListNode{ next, val } = *b;
                Some(Box::new(ListNode{
                    val,
                    next: Solution::delete_duplicates_aux(next, val)
                }))
            }
        }
    }
    pub fn delete_duplicates_aux(head: Option<Box<ListNode>>, prev: i32)
        -> Option<Box<ListNode>> {
        match head {
            None => None,
            Some(b) => {
                let ListNode{ next, val } = *b;
                if prev == val {
                    return Solution::delete_duplicates_aux(next, val)
                }
                Some(Box::new(ListNode {
                    val,
                    next: Solution::delete_duplicates_aux(next, val)
                }))
            }
        }
    }
}

fn main() {
    let li = ListNode::new(1);
    let li = ListNode{ val: 2, next: Some(Box::new(li))};
    let li = ListNode{ val: 2, next: Some(Box::new(li))};
    let li = ListNode{ val: 2, next: Some(Box::new(li))};
    let li = ListNode{ val: 3, next: Some(Box::new(li))};
    let li = ListNode{ val: 2, next: Some(Box::new(li))};
    let li = ListNode{ val: 1, next: Some(Box::new(li))};
    dbg!(&li);
    dbg!(Solution::delete_duplicates(Some(Box::new(li))));
}
