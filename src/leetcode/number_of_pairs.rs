use std::collections::HashMap;

/// 3162. 优质数对的总数I 、 3164. 优质数对的总数II
/// 给你两个整数数组 nums1 和 nums2，长度分别为 n 和 m。同时给你一个正整数 k。
/// 如果 nums1[i] 可以被 nums2[j] * k 整除，则称数对 (i, j) 为 优质数对（0 <= i <= n - 1, 0 <= j <= m - 1）。
/// 返回 优质数对 的总数。
pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
    let mut total = 0;
    for n in nums1 {
        for m in &nums2 {
            if n % (m * k) == 0 {
                total += 1;
            }
        }
    }
    total
}

pub fn number_of_pairs_ii(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
    let mut total = 0;
    let mut nums1_map = HashMap::new();
    let mut nums2_map = HashMap::new();
    let mut max = 0;
    for n in nums1 {
        max = max.max(n);
        nums1_map.entry(n).and_modify(|f| *f += 1).or_insert(1);
    }
    for n in nums2 {
        nums2_map.entry(n).and_modify(|f| *f += 1).or_insert(1);
    }
    for m in nums2_map.iter() {
        let mut n = 1;
        loop {
            let num = m.0 * k * n;
            if nums1_map.contains_key(&num) {
                total += m.1 * nums1_map.get(&num).unwrap();
            }
            if num > max {
                break;
            }
            n += 1;
        }
    }
    total
}

#[test]
fn test_1() {
    assert_eq!(number_of_pairs_ii(vec![1, 3, 4], vec![1, 3, 4], 1), 5);
}

#[test]
fn test_2() {
    assert_eq!(number_of_pairs_ii(vec![1, 2, 4, 12], vec![2, 4], 3), 2);
}
