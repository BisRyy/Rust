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
    pub fn leaf_similar(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn collect_leaves(n: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
            match n {
                None => vec![],
                Some(n) => {
                    if n.borrow().left.is_none() && n.borrow().right.is_none() {
                        return vec![n.borrow().val];
                    }
                    let mut list = collect_leaves(n.borrow().left.clone());
                    list.extend(collect_leaves(n.borrow().right.clone()));
                    list
                }
            }
        }
        collect_leaves(root1) == collect_leaves(root2)
    }
}