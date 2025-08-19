// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
use std::rc::Rc;
use std::cell::RefCell;

pub fn amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {

    let mut current_deep = 0;
    let mut max_deep = 0;
    let mut current_deep = 0;


    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, start: i32, current_deep: &mut i32, max_deep: &mut i32) {
        if let Some(n) = node {
            let n = n.borrow();
            if n.val == start {
                *current_deep = (*max_deep).max(*current_deep);
            } else {
                *current_deep += 1;
                dfs(n.left.clone(), start, current_deep, max_deep);
                dfs(n.right.clone(), start, current_deep, max_deep);
                *current_deep -= 1;
            }
        }
    }


    0
}