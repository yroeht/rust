/* https://leetcode.com/problems/add-two-numbers/ 2 */

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
}

impl Solution {
    pub fn decompose(l: Option<Box<ListNode>>) -> (i32, Option<Box<ListNode>>) {
        match l {
            None => (0, None),
            Some(ln) => {
                let ListNode{val, next} = *ln;
                (val, next)
            }
        }
    }

    pub fn add_two_numbers_aux(l1: Option<Box<ListNode>>,
                               l2: Option<Box<ListNode>>,
                               carry: i32) -> Option<Box<ListNode>> {
        let (l1h, l1t) = Solution::decompose(l1);
        let (l2h, l2t) = Solution::decompose(l2);
        let interm = l1h + l2h + carry;
        let (newhead, newcarry) = (interm % 10, interm / 10);
        match (&l1t, &l2t) {
            (&None, &None) => match (newhead, newcarry) {
                (0, 0) => None,
                (h, c) => Some(Box::new(ListNode{val: h, next: Solution::add_two_numbers_aux(None, None, c)}))
            }
            _ => Some(Box::new(ListNode{val: newhead, next: Solution::add_two_numbers_aux(l1t, l2t, newcarry)}))
        }
    }

    pub fn add_two_numbers(l1: Option<Box<ListNode>>,
                           l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let res = Solution::add_two_numbers_aux(l1, l2, 0);
        match &res {
            &None => Some(Box::new(ListNode{val: 0, next: None})),
            _ => res
        }
    }
}

fn main() {
    let l1 = Some(Box::new(ListNode{ val : 3, next : None}));
    let l1 = Some(Box::new(ListNode{ val : 4, next : l1}));
    let l1 = Some(Box::new(ListNode{ val : 2, next : l1}));
    let l2 = Some(Box::new(ListNode{ val : 4, next : None}));
    let l2 = Some(Box::new(ListNode{ val : 6, next : l2}));
    let l2 = Some(Box::new(ListNode{ val : 5, next : l2}));
    dbg!(Solution::add_two_numbers(l1, l2));
    let l1 = Some(Box::new(ListNode{ val : 0, next : None}));
    let l2 = Some(Box::new(ListNode{ val : 0, next : None}));
    dbg!(Solution::add_two_numbers(l1, l2));
    let l1 = Some(Box::new(ListNode{ val : 9, next : None}));
    let l1 = Some(Box::new(ListNode{ val : 9, next : l1}));
    let l1 = Some(Box::new(ListNode{ val : 9, next : l1}));
    let l1 = Some(Box::new(ListNode{ val : 9, next : l1}));
    let l1 = Some(Box::new(ListNode{ val : 9, next : l1}));
    let l1 = Some(Box::new(ListNode{ val : 9, next : l1}));
    let l1 = Some(Box::new(ListNode{ val : 9, next : l1}));
    let l2 = Some(Box::new(ListNode{ val : 9, next : None}));
    let l2 = Some(Box::new(ListNode{ val : 9, next : l2}));
    let l2 = Some(Box::new(ListNode{ val : 9, next : l2}));
    let l2 = Some(Box::new(ListNode{ val : 9, next : l2}));
    dbg!(Solution::add_two_numbers(l1, l2));
}
