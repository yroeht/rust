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

fn depth(head: &Option<Box<ListNode>>) -> i32 {
    match head {
        None => 1,
        Some(ln) => {
            let ListNode{ val: _, ref next } = **ln;
            1 + depth(next)
        }
    }
}

fn pick(head: Option<Box<ListNode>>, depth: i32) -> Option<Box<ListNode>> {
    match (head, depth) {
        (head, 1) => head,
        (Some(ln), n) => {
            let ListNode { val : _, next } = *ln;
            pick(next, n - 1)
        },
        _ => panic!()
    }
}

fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let d: i32 = depth(&head);
    dbg!(d);
    pick(head, (d as f32 / 2f32).ceil() as i32)
}

fn main() {
    let li = ListNode::new(1);
    let li = ListNode{ val: 2, next: Some(Box::new(li))};
    let li = ListNode{ val: 3, next: Some(Box::new(li))};
    let li = ListNode{ val: 2, next: Some(Box::new(li))};
    let li = ListNode{ val: 1, next: Some(Box::new(li))};
    dbg!(&li);
    dbg!(middle_node(Some(Box::new(li))));
}
