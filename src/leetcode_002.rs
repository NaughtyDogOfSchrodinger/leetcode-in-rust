use crate::common::ListNode;

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    fn carried(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        mut carry: i32,
    ) -> Option<Box<ListNode>> {
        if l1.is_none() && l2.is_none() && carry == 0 {
            None
        } else {
            Some(Box::new(ListNode {
                next: carried(
                    l1.and_then(|n1| {
                        carry += n1.val;
                        n1.next
                    }),
                    l2.and_then(|n2| {
                        carry += n2.val;
                        n2.next
                    }),
                    carry / 10,
                ),
                val: carry % 10,
            }))
        }
    }
    carried(l1, l2, 0)
}

#[cfg(test)]
mod test {
    use crate::common::ListNode;
    use crate::leetcode_002::add_two_numbers;

    #[test]
    fn test() {
        let node1 = Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode { val: 9, next: None })),
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
        let mut r = add_two_numbers(node1, node2);
        while let Some(node) = r.take() {
            print!("{}->", node.val);
            r = node.next;
        }
        println!();
    }
}
