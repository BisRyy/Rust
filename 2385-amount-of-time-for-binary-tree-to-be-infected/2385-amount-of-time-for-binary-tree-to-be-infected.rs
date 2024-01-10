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
    pub fn amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {
        let (_, distance) = Self::_amount_of_time(root, start);
        distance
    }

    fn _amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> (Option<i32>, i32) {
        if let Some(root) = root {
            let root = root.borrow();
            let (left_start_distance, left_max_distance) =
                Self::_amount_of_time(root.left.clone(), start);
            let (right_start_distance, right_max_distance) =
                Self::_amount_of_time(root.right.clone(), start);

            if root.val == start {
                (Some(0), i32::max(left_max_distance, right_max_distance) + 1)
            } else {
                if let Some(left_start_distance) = left_start_distance {
                    (
                        Some(left_start_distance + 1),
                        i32::max(
                            right_max_distance + left_start_distance + 2,
                            left_max_distance,
                        ),
                    )
                } else if let Some(right_start_distance) = right_start_distance {
                    (
                        Some(right_start_distance + 1),
                        i32::max(
                            left_max_distance + right_start_distance + 2,
                            right_max_distance,
                        ),
                    )
                } else {
                    (None, i32::max(left_max_distance, right_max_distance) + 1)
                }
            }
        } else {
            (None, -1)
        }

    }
}