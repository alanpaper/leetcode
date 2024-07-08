use std::cell::RefCell;
use std::collections::VecDeque;
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
    let mut ans = vec![];
    println!("{:?}", ans);

    Some(Rc::new(RefCell::new(TreeNode {
        val: todo!(),
        left: todo!(),
        right: todo!(),
    })))
}

fn bfs(root: Option<Rc<RefCell<TreeNode>>>) {
    let mut queue = VecDeque::new();
    queue.push_back(root);


    while !queue.is_empty() {
        if let Some(node) = queue.pop_front() {
            if let Some(n) = node {
                
            }
        }
    }

}
