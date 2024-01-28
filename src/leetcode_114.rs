use crate::common::TreeNode;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
    if let Some(node) = root {
        let mut n = node.as_ptr();
        unsafe {
            flatten(&mut (*n).left);
            flatten(&mut (*n).right);
            let temp = (*n).right.clone();
            (*n).right = (*n).left.clone();
            (*n).left = None;
            while let Some(nn) = (*n).right.clone() {
                n = nn.as_ptr();
            }
            (*n).right = temp;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::common::TreeNode;
    use crate::leetcode_114::flatten;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test() {
        let mut node = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        flatten(&mut node);
        println!("{:?}", node);
    }
}
