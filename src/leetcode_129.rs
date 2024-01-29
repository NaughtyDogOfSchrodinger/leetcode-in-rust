use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn sum_numbers(root: &Option<Rc<RefCell<TreeNode>>>, path: i32) -> i32 {
        match root {
            None => path,
            Some(node) => {
                let bo = node.borrow();
                match (&bo.left, &bo.right) {
                    (None, None) => sum_numbers(&bo.left, path * 10 + bo.val),
                    (Some(_), None) => sum_numbers(&bo.left, path * 10 + bo.val),
                    (None, Some(_)) => sum_numbers(&bo.right, path * 10 + bo.val),
                    (Some(_), Some(_)) => {
                        sum_numbers(&bo.left, path * 10 + bo.val)
                            + sum_numbers(&bo.right, path * 10 + bo.val)
                    }
                }
            }
        }
    }
    sum_numbers(&root, 0)
}

#[cfg(test)]
mod test {
    use crate::common::TreeNode;
    use crate::leetcode_129::sum_numbers;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test() {
        let node = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: None,
                right: None,
            }))),
        })));
        println!("{:?}", sum_numbers(node));
        // println!("{:?}", has_path_sum(None, 0));
        let node = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        })));
        // println!("{:?}", sum_numbers(node));
    }
}
