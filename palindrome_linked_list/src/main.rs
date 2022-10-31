/* https://leetcode.com/problems/palindrome-linked-list/ 234 */

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

fn reverse(li: &Option<Box<ListNode>>, acc: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match li {
        None => acc,
        Some(e) => {
            let ListNode{next, val} = &**e;
            reverse(&next, Some(Box::new(ListNode{val: *val, next: acc})))
        }
    }
}

pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    is_palindrome_aux(&head, reverse(&head, None))
}

fn is_palindrome_aux(li1: &Option<Box<ListNode>>, li2: Option<Box<ListNode>>) -> bool {
    match (li1, li2) {
        (None, None) => true,
        (Some(e1), Some(e2)) => {
            let ListNode{next: next1, val: val1} = &**e1;
            let ListNode{next: next2, val: val2} = *e2;
            val1 == &val2 && is_palindrome_aux(next1, next2)
        }
        _ => panic!("length mismatch")
    }
}

fn main() {
    let li = ListNode::new(1);
    let li = ListNode{ val: 1, next: Some(Box::new(li))};
    let li = ListNode{ val: 2, next: Some(Box::new(li))};
    let li = ListNode{ val: 2, next: Some(Box::new(li))};
    let li = ListNode{ val: 1, next: Some(Box::new(li))};
    dbg!(&li);
    dbg!(is_palindrome(Some(Box::new(li))));
    let li = ListNode::new(1);
    let li = ListNode{ val: 2, next: Some(Box::new(li))};
    let li = ListNode{ val: 3, next: Some(Box::new(li))};
    let li = ListNode{ val: 2, next: Some(Box::new(li))};
    let li = ListNode{ val: 1, next: Some(Box::new(li))};
    dbg!(&li);
    dbg!(is_palindrome(Some(Box::new(li))));
}
