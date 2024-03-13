use std::cell::RefCell;
use std::rc::Rc;

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

pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut ans = vec![];
    dfs(root, &mut ans);
    ans
}

fn dfs(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
    if let Some(node) = root {
        ans.push(node.borrow().val);
        dfs(node.borrow().left.clone(), ans);
        dfs(node.borrow().right.clone(), ans);
    }
}

#[test]
fn test_preorder_traversal() {
    let sub_tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
            right: None,
        }))),
    })));
    assert_eq!(preorder_traversal(sub_tree), vec![1, 2, 3]);
}
