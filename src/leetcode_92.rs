use crate::common::ListNode;

pub fn reverse_between(
    head: Option<Box<ListNode>>,
    left: i32,
    right: i32,
) -> Option<Box<ListNode>> {
    let mut dummy = Some(Box::new(ListNode {
        val: -1,
        next: head,
    }));
    let mut ptr = &mut dummy;

    for _ in 0..left - 1 {
        ptr = &mut ptr.as_mut().unwrap().next;
    }
    if let Some(mut sub) = ptr.as_mut().unwrap().next.take() {
        let mut tail = sub.next.take();

        for _ in left..right {
            let mut node = tail.unwrap();
            tail = node.next.replace(sub);
            sub = node;
        }

        ptr.as_mut().unwrap().next = Some(sub);

        for _ in left..=right {
            ptr = &mut ptr.as_mut().unwrap().next;
        }
        ptr.as_mut().unwrap().next = tail;
    }
    dummy.unwrap().next
}

#[cfg(test)]
mod test {
    use crate::common::ListNode;
    use crate::leetcode_92::reverse_between;

    #[test]
    fn test() {
        let node1 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 5, next: None })),
                    })),
                })),
            })),
        }));

        let node2 = Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode { val: 9, next: None })),
            })),
        }));
        let mut r = reverse_between(node1, 2, 4);
        while let Some(node) = r.take() {
            print!("{}->", node.val);
            r = node.next;
        }
        println!();
    }
}
