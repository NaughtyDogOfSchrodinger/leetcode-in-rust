use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (p, q) {
        (None, None) => true,
        (Some(p), Some(q)) => {
            let (mut p, mut q) = (p.borrow_mut(), q.borrow_mut());
            p.val == q.val
                && is_same_tree(p.left.take(), q.left.take())
                && is_same_tree(p.right.take(), q.right.take())
        }
        _ => false,
    }
}
