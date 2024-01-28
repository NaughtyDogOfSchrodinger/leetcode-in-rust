use crate::common::ListNode;

pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
    let mut left_node = Some(Box::new(ListNode {
        val: -1,
        next: None,
    }));
    let mut right_node = Some(Box::new(ListNode {
        val: -1,
        next: None,
    }));

    let (mut left, mut right) = (&mut left_node, &mut right_node);
    let mut head = head;
    while let Some(node) = head.take() {
        if node.val < x {
            left.as_mut().unwrap().next = Some(Box::new(ListNode {
                val: node.val,
                next: None,
            }));
            left = &mut left.as_mut().unwrap().next;
        } else {
            right.as_mut().unwrap().next = Some(Box::new(ListNode {
                val: node.val,
                next: None,
            }));
            right = &mut right.as_mut().unwrap().next;
        }
        head = node.next;
    }
    left.as_mut().unwrap().next = right_node.unwrap().next;
    left_node.unwrap().next
}

#[cfg(test)]
mod test {
    use crate::common::ListNode;
    use crate::leetcode_86::partition;

    #[test]
    fn test() {
        let node = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 2,
                            next: Some(Box::new(ListNode {
                                val: 5,
                                next: Some(Box::new(ListNode { val: 2, next: None })),
                            })),
                        })),
                    })),
                })),
            })),
        }));
        println!("{:?}", partition(node, 3));

        let node = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None })),
        }));
        // println!("{:?}", rotate_right(node, 2));
    }
}
