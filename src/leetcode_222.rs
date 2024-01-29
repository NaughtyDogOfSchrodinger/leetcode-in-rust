use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn count_nodes(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let bo = node.borrow();
                1 + count_nodes(&bo.left) + count_nodes(&bo.right)
            }
        }
    }
    count_nodes(&root)
}

#[cfg(test)]
mod test {
    use crate::common::TreeNode;
    use crate::leetcode_222::count_nodes;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test() {
        let node = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
        })));
        println!("{:?}", count_nodes(node));
    }
}
