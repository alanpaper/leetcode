/// 82. 删除排序链表中的重复元素 II
/// 给定一个已排序的链表的头 head ， 删除原始链表中所有重复数字的节点，只留下不同的数字 。返回 已排序的链表。
///
// Definition for singly-linked list.
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
pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut tmp = ListNode::new(0);
    let mut last = None;
    let mut head = head;
    let mut r = &mut tmp.next;
    while let Some(mut x) = head {
        head = x.next.take();
        let t = x.val;
        if Some(t) != last && head.as_ref().map_or(true, |y| y.val != t) {
            r = &mut r.get_or_insert(x).next;
        }
        last = Some(t);
    }
    tmp.next
}

#[test]
fn test_delete_duplicates() {
    let link = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 5, next: None })),
                    })),
                })),
            })),
        })),
    }));

    let sub_link = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode { val: 5, next: None })),
            })),
        })),
    }));

    assert_eq!(delete_duplicates(link), sub_link);
}
