use crate::common::ListNode;

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    fn merge(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (l1, l2, val) = match (list1, list2) {
            (None, None) => return None,
            (Some(n1), Some(n2)) => {
                if n1.val < n2.val {
                    (n1.next, Some(n2), n1.val)
                } else {
                    (Some(n1), n2.next, n2.val)
                }
            }
            (Some(n1), None) => (n1.next, None, n1.val),
            (None, Some(n2)) => (None, n2.next, n2.val),
        };

        Some(Box::new(ListNode {
            val,
            next: merge(l1, l2),
        }))
    }
    merge(list1, list2)
}
