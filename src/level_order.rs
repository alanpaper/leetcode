use std::{borrow::BorrowMut, cell::RefCell, collections::VecDeque, rc::Rc};
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

/// 102. 二叉树的层序遍历
pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut ans = VecDeque::new();
    let mut cur = VecDeque::new();
    if let Some(x) = root {
        cur.push(x);
    }
    while !cur.is_empty() {
        let mut nxt = Vec::new();
        let mut vals = Vec::with_capacity(cur.len());
        for node in cur {}
        cur = nxt;
        ans.push(vals);
    }
    ans
}
