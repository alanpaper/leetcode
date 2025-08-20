use std::collections::{BinaryHeap};
/// 373
/// 
use std::cmp::{Ordering, Reverse};

#[derive(Eq, PartialEq, PartialOrd)]
struct HeapElem(i32, usize, usize);

impl Ord for HeapElem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
    let mut ans = vec![];
    let mut bh = BinaryHeap::new();
    bh.push(Reverse(HeapElem(nums1[0] + nums2[0], 0, 0)));
    while ans.len() < k as usize {
        if let Some(Reverse(HeapElem(_, i, j))) = bh.pop() {
            ans.push(vec![nums1[i], nums2[j]]);
            if j == 0 && i + 1 < nums1.len() {
                bh.push(Reverse(HeapElem(nums1[i + 1] + nums2[j], i + 1, 0)));
            } 
            if j + 1 < nums2.len() {
                bh.push(Reverse(HeapElem(nums1[i] + nums2[j + 1], i, j + 1)));
            }
        } else {
            break;
        }
    }
    ans
}

#[test]
fn test_1() {
    let nums1 = vec![1,7,11];
    let nums2 = vec![2,4,6];
    assert_eq!(k_smallest_pairs(nums1, nums2, 3), vec![
        vec![1,2],
        vec![1,4],
        vec![1,6]
    ])
}

#[test]
fn test_2() {
    let nums1 = vec![1,1,2];
    let nums2 = vec![1,2,3];
    assert_eq!(k_smallest_pairs(nums1, nums2, 2), vec![
        vec![1,1],
        vec![1,1],
    ])
}