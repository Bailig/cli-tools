pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// https://leetcode.com/problems/reverse-linked-list-ii/
fn reverse_between(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
    let mut before_head = Some(Box::new(ListNode { val: 0, next: head }));

    let initial = {
        let mut node = &mut before_head;
        for _ in 1..left {
            node = &mut node.as_mut()?.next;
        }
        node
    };

    let (mut mid_head, tail) = {
        let mut node = initial.as_mut()?.next.take();
        let mut node2 = node.as_mut()?.next.take();
        for _ in left..right {
            let node3 = node2.as_mut()?.next.take();
            node2.as_mut()?.next = node;
            node = node2;
            node2 = node3;
        }
        (node, node2)
    };

    let mid_last = {
        let mut node = &mut mid_head;
        for _ in left..right {
            node = &mut node.as_mut()?.next;
        }
        node
    };

    mid_last.as_mut()?.next = tail;
    initial.as_mut()?.next = mid_head;

    before_head.unwrap().next
}

pub fn run() {
    let mut node1 = ListNode::new(1);
    let mut node2 = ListNode::new(2);
    let mut node3 = ListNode::new(3);
    let mut node4 = ListNode::new(4);
    let node5 = ListNode::new(5);
    node4.next = Some(Box::new(node5));
    node3.next = Some(Box::new(node4));
    node2.next = Some(Box::new(node3));
    node1.next = Some(Box::new(node2));

    reverse_between(Some(Box::new(node1)), 1, 2);
}
