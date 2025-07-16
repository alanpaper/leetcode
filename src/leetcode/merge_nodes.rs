use std::borrow::BorrowMut;

/// 2181. 合并零之间的节点
/// 给你一个链表的头节点 head ，该链表包含由 0 分隔开的一连串整数。链表的 开端 和 末尾 的节点都满足 Node.val == 0 。
/// 对于每两个相邻的 0 ，请你将它们之间的所有节点合并成一个节点，其值是所有已合并节点的值之和。
/// 然后将所有 0 移除，修改后的链表不应该含有任何 0 。
/// 返回修改后链表的头节点 head 。
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut ans = vec![];
    if let Some(link) = head {
        let mut node = link;
        let mut total = 0;
        loop {
            if node.val == 0 {
                if total > 0 {
                    ans.push(total);
                }
                total = 0;
            } else {
                total += node.val
            }
            if node.next.is_none() {
                break;
            }
            node = node.next?;
        }
        if !ans.is_empty() {
            let mut h = Box::new(ListNode::new(ans[0]));
            for i in 1..ans.len() {
                let mut head: &mut ListNode = h.borrow_mut();
                head.next = Some(Box::new(ListNode::new(ans[i])));
                
            }
            return Some(h);
        } else {
            return None;
        }
    }
    None
}
