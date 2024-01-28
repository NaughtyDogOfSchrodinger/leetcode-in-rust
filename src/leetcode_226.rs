use crate::common::TreeNode;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        None => None,
        Some(node) => {
            let mut n = node.borrow_mut();
            Some(Rc::new(RefCell::new(TreeNode {
                val: n.val,
                left: invert_tree(n.right.take()),
                right: invert_tree(n.left.take()),
            })))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::common::TreeNode;
    use crate::leetcode_226::invert_tree;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test() {
        let node = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 15,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        println!("{:?}", invert_tree(node));
    }
}
