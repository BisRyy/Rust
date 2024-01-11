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
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, mn: i32, mx: i32) -> i32 {
            if let Some(node) = node {
                let node = node.borrow();
                let v = node.val;
                let res = if mn == i32::MAX { 0 } else { (v - mn).abs().max((v - mx).abs()) };
                let left = dfs(node.left.clone(), v.min(mn), v.max(mx));
                let right = dfs(node.right.clone(), v.min(mn), v.max(mx));
                return res.max(left).max(right);
            }
            0
        }
        dfs(root, i32::MAX, i32::MIN)
    }
}