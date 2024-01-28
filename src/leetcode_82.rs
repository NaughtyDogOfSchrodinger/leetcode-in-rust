use crate::common::ListNode;

pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        None
    } else {
        let mut dummy = ListNode {
            val: -1,
            next: None,
        };
        let mut root = &mut dummy;
        let mut cur = head;
        while let Some(mut node) = cur.take() {
            let mut next = &mut node.next;

            let val = node.val;
            let mut count = 0;
            while next.as_mut().is_some() && val == next.as_mut().unwrap().val {
                count += 1;
                next = &mut next.as_mut().unwrap().next;
            }
            if count > 0 {
                cur = next.take();
            } else {
                root.next = Some(Box::new(ListNode { val, next: None }));
                root = root.next.as_mut().unwrap();
                cur = node.next;
            }
        }
        dummy.next
    }
}

#[cfg(test)]
mod test {
    use crate::common::ListNode;
    use crate::leetcode_82::delete_duplicates;

    #[test]
    fn test() {
        let node = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode {
                                val: 4,
                                next: Some(Box::new(ListNode { val: 5, next: None })),
                            })),
                        })),
                    })),
                })),
            })),
        }));
        println!("{:?}", delete_duplicates(node));
        let node = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 1, next: None })),
        }));
        println!("{:?}", delete_duplicates(node));
        let node = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None })),
        }));
        println!("{:?}", delete_duplicates(node));
    }
}
