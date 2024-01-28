use crate::common::ListNode;

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    if head.is_none() {
        None
    } else {
        let mut num = n;
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut slow = &mut dummy as *mut Box<ListNode>;
        let mut fast = &mut dummy as *mut Box<ListNode>;
        unsafe {
            while num > 0 {
                fast = (*fast).next.as_mut().unwrap();
                num -= 1;
            }
        }

        if num > 0 {
            None
        } else {
            unsafe {
                while (*slow).next.is_some() && (*fast).next.is_some() {
                    slow = (*slow).next.as_mut().unwrap();
                    fast = (*fast).next.as_mut().unwrap();
                }
                (*slow).next = (*slow).next.take().unwrap().next;
            }

            dummy.next
        }
    }
}

pub fn remove_nth_from_end1(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut dummy = Some(Box::new(ListNode {
        val: -1,
        next: head,
    }));
    let mut fast = &mut dummy.clone();
    let mut slow = &mut fast.clone();
    for _ in 0..n + 1 {
        fast = &mut fast.as_mut().unwrap().next;
    }
    while let Some(_) = &mut fast.as_mut() {
        fast = &mut fast.as_mut().unwrap().next;
        slow = &mut slow.as_mut().unwrap().next;
    }
    if let Some(Some(n)) = slow.as_mut().unwrap().next.take().map(|node| node.next) {
        slow.as_mut().unwrap().next.replace(n);
    }
    dummy.unwrap().next
}

#[cfg(test)]
mod test {
    use crate::common::ListNode;
    use crate::leetcode_19::{remove_nth_from_end, remove_nth_from_end1};

    #[test]
    fn test() {
        let node = Some(Box::new(ListNode {
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
        println!("{:?}", remove_nth_from_end1(node, 2));
        let node = Some(Box::new(ListNode { val: 1, next: None }));
        println!("{:?}", remove_nth_from_end1(node, 1));
        let node = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None })),
        }));
        println!("{:?}", remove_nth_from_end1(node, 1));
    }
}
