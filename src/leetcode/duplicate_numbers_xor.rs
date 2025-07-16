use std::collections::HashSet;

/// 3158. 求出现两次数字的XOR值
/// 给你一个数组 nums ，数组中的数字 要么 出现一次，要么 出现两次。
/// 请你返回数组中所有出现两次数字的按位 XOR 值，如果没有数字出现过两次，返回 0 。
pub fn duplicate_numbers_xor(nums: Vec<i32>) -> i32 {
    let mut nums_set = HashSet::new();
    let mut ans = 0;
    for n in nums {
        if nums_set.contains(&n) {
            ans ^= n;
        }
        nums_set.insert(n);
    }
    ans
}

#[test]
fn test_1() {
    assert_eq!(duplicate_numbers_xor(vec![1, 2, 1, 3]), 1);
}

#[test]
fn test_2() {
    assert_eq!(duplicate_numbers_xor(vec![1, 2, 3]), 0);
}

#[test]
fn test_3() {
    assert_eq!(duplicate_numbers_xor(vec![1, 2, 2, 1]), 3);
}
