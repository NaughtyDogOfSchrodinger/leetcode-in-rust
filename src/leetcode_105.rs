use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    assert_eq!(preorder.len(), inorder.len());
    fn build_tree(
        preorder: &Vec<i32>,
        root_index: usize,
        inorder: &Vec<i32>,
        start: usize,
        end: usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if start == end {
            return Some(Rc::new(RefCell::new(TreeNode {
                val: inorder[start],
                left: None,
                right: None,
            })));
        }
        if start > end || root_index >= preorder.len() {
            return None;
        }
        let val = preorder[root_index];

        if inorder[start] == val {
            //left is none, root_index + 1 in right root index
            Some(Rc::new(RefCell::new(TreeNode {
                val,
                left: None,
                right: build_tree(preorder, root_index + 1, inorder, start + 1, end),
            })))
        } else if inorder[end] == val {
            //right is none, root_index + 1 in right root index
            Some(Rc::new(RefCell::new(TreeNode {
                val,
                left: build_tree(preorder, root_index + 1, inorder, start, end - 1),
                right: None,
            })))
        } else {
            let mid = (start..=end).find(|&index| inorder[index] == val);
            assert!(mid.is_some());
            let mid = mid.unwrap();
            Some(Rc::new(RefCell::new(TreeNode {
                val,
                //root_index + 1 in right root index
                left: build_tree(preorder, root_index + 1, inorder, start, mid - 1),
                //root_index + 1 + (left child tree count) in right root index
                right: build_tree(
                    preorder,
                    root_index + 1 + mid - start,
                    inorder,
                    mid + 1,
                    end,
                ),
            })))
        }
    }
    match preorder.len() {
        0 => None,
        len => build_tree(&preorder, 0, &inorder, 0, len - 1),
    }
}

#[cfg(test)]
mod test {
    use crate::leetcode_105::build_tree;

    #[test]
    fn test() {
        let (preorder, inorder) = (vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]);
        println!("{:?}", build_tree(preorder, inorder));
        let (preorder, inorder) = (vec![1, 2, 3, 4], vec![1, 2, 3, 4]);
        println!("{:?}", build_tree(preorder, inorder));
    }
}
