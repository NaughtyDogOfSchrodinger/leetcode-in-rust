use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    fn has_path_sum(root: &Option<Rc<RefCell<TreeNode>>>, sum: i32, target_sum: i32) -> bool {
        match root {
            None => sum == target_sum,
            Some(node) => {
                let borrow = node.borrow();
                has_path_sum(&borrow.left, sum + borrow.val, target_sum)
                    || has_path_sum(&borrow.right, sum + borrow.val, target_sum)
            }
        }
    }
    if root.is_none() {
        false
    } else {
        has_path_sum(&root, 0, target_sum)
    }
}

#[cfg(test)]
mod test {
    use crate::common::TreeNode;
    use crate::leetcode_112::has_path_sum;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test() {
        let node = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 11,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 7,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: None,
                    }))),
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 8,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 13,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
        })));
        // println!("{:?}", has_path_sum(node, 22));
        // println!("{:?}", has_path_sum(None, 0));
        let node = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
            right: None,
        })));
        println!("{:?}", has_path_sum(node, 1));
    }
}
