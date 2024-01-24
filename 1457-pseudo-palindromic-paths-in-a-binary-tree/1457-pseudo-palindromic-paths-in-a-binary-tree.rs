// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn pseudo_palindromic_paths (root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, mut path: i32) -> i32 {
            if let Some(node) = node {
                let node = node.borrow();
                path ^= 1 << node.val;
                if node.left.is_none() && node.right.is_none() {
                    return if path.count_ones() <= 1 { 1 } else { 0 };
                }
                return dfs(node.left.clone(), path) + dfs(node.right.clone(), path);
            }
            0
        }
        dfs(root, 0)
    }
}