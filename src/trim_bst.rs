/// 669. 修剪二叉搜索树
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
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
pub fn trim_bst(
    root: Option<Rc<RefCell<TreeNode>>>,
    low: i32,
    high: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    let mut r = root.clone().unwrap();

    dfs(&mut r, low, high, &mut r);

    Some(r)
}

fn dfs(node: &mut Rc<RefCell<TreeNode>>, low: i32, high: i32, parent: &mut Rc<RefCell<TreeNode>>) {
    if let n = node {
        if n.borrow_mut().val < low || n.borrow_mut().val > high {
            if let Some(mut left) = n.borrow_mut().left {
                dfs(&mut left, low, high, n);
            }
            if let Some(mut right) = n.borrow_mut().right {
                dfs(&mut right, low, high, n);
            }
        } else {
            parent.borrow_mut().left = n.borrow_mut().left;
            parent.borrow_mut().right = n.borrow_mut().right;
            if let Some(mut left) = parent.borrow_mut().left {
                dfs(&mut left, low, high, parent);
            }
            if let Some(mut right) = parent.borrow_mut().right {
                dfs(&mut right, low, high, parent);
            }
        }
    }
}
