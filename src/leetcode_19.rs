// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    if head.is_none() {
        None
    } else {
        let mut num = n;
        let mut dummy = Box::new(ListNode {val: 0, next: head});
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
                while (*slow).next.is_some() && (*fast).next.is_some(){
                    slow = (*slow).next.as_mut().unwrap();
                    fast = (*fast).next.as_mut().unwrap();
                }
                (*slow).next = (*slow).next.take().unwrap().next;
            }

            dummy.next
        }
    }

}



#[cfg(test)]
mod test {
    use crate::leetcode_19::{ListNode, remove_nth_from_end};

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
                        next: Some(Box::new(ListNode {
                            val: 5,
                            next: None
                        }))
                    }))
                }))
            }))
        }));
        println!("{:?}", remove_nth_from_end(node, 2));
        let node = Some(Box::new(ListNode {
            val: 1,
            next: None}));
        println!("{:?}", remove_nth_from_end(node, 1));
        let node = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: None}))}));
        println!("{:?}", remove_nth_from_end(node, 1));
    }
}