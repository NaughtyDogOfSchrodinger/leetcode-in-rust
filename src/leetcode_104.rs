use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

// impl Solution {
pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            let mut node = node.borrow_mut();
            1 + max_depth(node.left.take()).max(max_depth(node.right.take()))
        }
    }
}
// }

#[cfg(test)]
mod test {
    use crate::common::TreeNode;
    use crate::leetcode_104::max_depth;
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
        println!("{:?}", max_depth(node));
    }
}
