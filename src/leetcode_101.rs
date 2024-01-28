use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn is_symmetric(
        left: &Option<Rc<RefCell<TreeNode>>>,
        right: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (left, right) {
            (None, None) => true,
            (Some(l), Some(r)) => {
                let l = l.borrow();
                let r = r.borrow();
                l.val == r.val && is_symmetric(&l.left, &r.right) && is_symmetric(&l.right, &r.left)
            }
            _ => false,
        }
    }

    match root {
        None => true,
        Some(node) => {
            let node = node.borrow();
            is_symmetric(&node.left, &node.right)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::common::TreeNode;
    use crate::leetcode_101::is_symmetric;
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

        let rr = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: None,
                right: None,
            }))),
        })));
        println!("{:?}", is_symmetric(rr));
    }
}
