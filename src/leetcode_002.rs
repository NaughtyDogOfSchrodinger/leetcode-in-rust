use std::ptr;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

pub fn carried(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, mut carry: i32) -> Option<Box<ListNode>> {
    if l1.is_none() && l2.is_none() && carry == 0 {
        None
    } else {
        Some(Box::new(ListNode {
            next: carried(
                l1.and_then(|x| {
                    carry += x.val;
                    x.next
                }),
                l2.and_then(|x| {
                    carry += x.val;
                    x.next
                }),
                carry / 10,
            ),
            val: carry % 10,
        }))
    }
}


pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    carried(l1, l2, 0)
}


#[cfg(test)]
mod test {
    use crate::leetcode_002::{add_two_numbers, ListNode};

    #[test]
    fn test() {
        let node1 = Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: None,
                    })),
                })),
            })),
        }));

        let node2 = Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: None,
                })),
            })),
        }));
        let mut r = add_two_numbers(node1, node2);
        while let Some(node) = r.take() {
            print!("{}->", node.val);
            r = node.next;
        }
        println!();
    }
}










