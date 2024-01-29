use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct BSTIterator {
    arr: Vec<i32>,
    cur: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        fn build(root: Option<Rc<RefCell<TreeNode>>>, mut arr: &mut Vec<i32>) {
            match root {
                None => return,
                Some(node) => {
                    let mut bo = node.borrow_mut();
                    build(bo.left.take(), arr);
                    arr.push(bo.val);
                    build(bo.right.take(), arr);
                }
            }
        }
        let mut arr = vec![];
        build(root, &mut arr);
        Self { arr, cur: 0 }
    }

    fn next(&mut self) -> i32 {
        let r = self.arr[self.cur];
        self.cur += 1;
        r
    }

    fn has_next(&self) -> bool {
        self.cur < self.arr.len()
    }
}
