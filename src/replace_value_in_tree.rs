use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
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

pub fn replace_value_in_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut map = HashMap::new();
}

fn grandfather(node: Option<Rc<RefCell<TreeNode>>>, map: HashMap<i32, (i32, i32)>) {
    if let Some(n) = node {
        if let Some(f) = n.borrow().left {
            s.borrow().left
        }
    }
}
