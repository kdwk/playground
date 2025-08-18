// https://leetcode.com/problems/add-two-numbers/submissions/
use stdext::prelude::*;

#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
    fn from_slice(s: &[i32]) -> Option<Box<Self>> {
        match s {
            [] => None,
            [elem, others @ ..] => Some(Box::new(ListNode {
                val: *elem,
                next: ListNode::from_slice(others),
            })),
        }
    }
}

fn _answer(
    carry: i32,
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (Some(node1), Some(node2)) => {
            let val1 = node1.val;
            let next1 = node1.next;
            let val2 = node2.val;
            let next2 = node2.next;
            let sum = val1 + val2 + carry;
            let next_carry = sum / 10;
            Some(Box::new(ListNode {
                val: sum % 10,
                next: _answer(next_carry, next1, next2),
            }))
        }
        (Some(node1), None) => {
            let ListNode { val, next } = *node1;
            let sum = val + carry;
            let next_carry = sum / 10;
            Some(Box::new(ListNode {
                val: sum % 10,
                next: _answer(next_carry, next, None),
            }))
        }
        (None, Some(node2)) => {
            let ListNode { val, next } = *node2;
            let sum = val + carry;
            let next_carry = sum / 10;
            Some(Box::new(ListNode {
                val: sum % 10,
                next: _answer(next_carry, None, next),
            }))
        }
        (None, None) => {
            if carry > 0 {
                Some(Box::new(ListNode {
                    val: carry,
                    next: None,
                }))
            } else {
                None
            }
        }
    }
}

pub fn answer(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    _answer(0, l1, l2)
}

pub fn test() {
    answer(
        ListNode::from_slice(&[9, 9, 9, 9, 9, 9, 9]),
        ListNode::from_slice(&[9, 9, 9, 9]),
    )
    .log();
}
