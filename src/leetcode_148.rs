use crate::common::ListNode;

pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut arr = Vec::<i32>::new();
    let mut head = head;
    while let Some(node) = head.take() {
        arr.push(node.val);
        head = node.next;
    }
    arr.sort();
    let mut dummy = ListNode {
        val: -1,
        next: None,
    };
    let mut cur = &mut dummy;
    for val in arr {
        cur.next = Some(Box::new(ListNode { val, next: None }));
        cur = cur.next.as_mut().unwrap();
    }
    dummy.next
}

pub fn sort_list1(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    fn merge(left: Option<Box<ListNode>>, right: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (left, right) {
            (None, None) => None,
            (Some(mut left), Some(mut right)) => {
                if left.val < right.val {
                    left.next = merge(left.next.take(), Some(right));
                    Some(left)
                } else {
                    right.next = merge(Some(left), right.next.take());
                    Some(right)
                }
            }
            (Some(node), None) | (None, Some(node)) => Some(node),
        }
    }
    if head.as_ref().is_none() || head.as_ref().unwrap().next.is_none() {
        return head;
    }
    let mut head = head;
    let mut all = &head;
    let mut half = &head;
    let mut half_len = 0;
    while half.as_ref().is_some() && all.as_ref().is_some() && all.as_ref().unwrap().next.is_some()
    {
        half = &half.as_ref().unwrap().next;
        all = &all.as_ref().unwrap().next.as_ref().unwrap().next;
        half_len += 1;
    }
    let mut half = &mut head;
    for _ in 0..half_len {
        half = &mut half.as_mut().unwrap().next;
    }
    let right = sort_list1(half.take());

    let left = sort_list1(head);
    merge(left, right)
}

#[cfg(test)]
mod test {
    use crate::common::ListNode;
    use crate::leetcode_148::{sort_list, sort_list1};

    #[test]
    fn test() {
        let node = Some(Box::new(ListNode {
            val: -1,
            next: Some(Box::new(ListNode {
                val: 5,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 0, next: None })),
                    })),
                })),
            })),
        }));
        println!("{:?}", sort_list1(node));
    }
}
