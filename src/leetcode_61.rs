use crate::common::ListNode;

pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut ptr = &head;
    if ptr.as_ref().is_none() || k == 0 {
        return head;
    }
    let mut count = 1;
    while ptr.as_ref().unwrap().next.is_some() {
        ptr = &ptr.as_ref().unwrap().next;
        count += 1;
    }
    let k = k % count;
    if count == 1 || k == 0 {
        return head;
    }
    let n = count - k;
    let mut dummy = Some(Box::new(ListNode {
        val: -1,
        next: head,
    }));

    let mut ptr = &mut dummy;

    for _ in 0..n {
        ptr = &mut ptr.as_mut().unwrap().next;
    }
    let mut new_head = ptr.as_mut().unwrap().next.take();
    let mut tail = &mut new_head;
    while tail.as_mut().is_some() && tail.as_mut().unwrap().next.is_some() {
        tail = &mut tail.as_mut().unwrap().next;
    }
    if tail.as_mut().is_some() {
        tail.as_mut().unwrap().next = dummy.unwrap().next;
    }

    new_head
}

#[cfg(test)]
mod test {
    use crate::common::ListNode;
    use crate::leetcode_61::rotate_right;

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
        println!("{:?}", rotate_right(node, 2));

        let node = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None })),
        }));
        println!("{:?}", rotate_right(node, 2));
    }
}
