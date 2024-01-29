use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    assert_eq!(postorder.len(), inorder.len());
    fn build_tree(
        postorder: &Vec<i32>,
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
        if start > end {
            return None;
        }
        let val = postorder[root_index];

        if inorder[start] == val {
            //left is none, root_index - 1 in right root index
            Some(Rc::new(RefCell::new(TreeNode {
                val,
                left: None,
                right: build_tree(postorder, root_index - 1, inorder, start + 1, end),
            })))
        } else if inorder[end] == val {
            //right is none, root_index - 1 in right root index
            Some(Rc::new(RefCell::new(TreeNode {
                val,
                left: build_tree(
                    postorder,
                    root_index - (end - end + 1),
                    inorder,
                    start,
                    end - 1,
                ),
                right: None,
            })))
        } else {
            let mid = (start..=end).find(|&index| inorder[index] == val);
            assert!(mid.is_some());
            let mid = mid.unwrap();
            Some(Rc::new(RefCell::new(TreeNode {
                val,
                //root_index - 1 in right root index
                left: build_tree(
                    postorder,
                    root_index - 1 - (end - mid),
                    inorder,
                    start,
                    mid - 1,
                ),
                //root_index - 1 - (left child tree count) in right root index
                right: build_tree(postorder, root_index - 1, inorder, mid + 1, end),
            })))
        }
    }
    match postorder.len() {
        0 => None,
        len => build_tree(&postorder, len - 1, &inorder, 0, len - 1),
    }
}

#[cfg(test)]
mod test {
    use crate::leetcode_106::build_tree;

    #[test]
    fn test() {
        let (inorder, postorder) = (vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3]);
        println!("{:?}", build_tree(inorder, postorder));
        let (inorder, postorder) = (vec![1, 2, 3, 4], vec![2, 1, 4, 3]);
        println!("{:?}", build_tree(inorder, postorder));
        let (inorder, postorder) = (vec![1, 2, 3, 4], vec![3, 2, 4, 1]);
        println!("{:?}", build_tree(inorder, postorder));
        let (inorder, postorder) = (
            vec![-4, -10, 3, -1, 7, 11, -8, 2],
            vec![-4, -1, 3, -10, 11, -8, 2, 7],
        );
        println!("{:?}", build_tree(inorder, postorder));
        let (inorder, postorder) = (vec![1, 2, 3, 4, 5, 6], vec![1, 4, 3, 6, 5, 2]);
        println!("{:?}", build_tree(inorder, postorder));
    }
}
