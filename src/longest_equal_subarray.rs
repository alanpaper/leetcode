use std::collections::HashMap;

/// 2831. 找出最长等值子数组
pub fn longest_equal_subarray(nums: Vec<i32>, k: i32) -> i32 {
    let mut ans = HashMap::new();
    let mut _ans = vec![];
    nums.iter().for_each(|f| {
        ans.entry(*f).and_modify(|e| *e += 1).or_insert(1);
    });
    ans.iter().for_each(|f| _ans.push((f.0, f.1)));
    _ans.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

    for num in _ans {}

    0
}

#[test]
fn test_1() {
    assert_eq!(longest_equal_subarray(vec![1, 3, 2, 3, 1, 3], 3), 3)
}

#[test]
fn test_2() {
    assert_eq!(longest_equal_subarray(vec![1, 1, 2, 2, 1, 1], 2), 2)
}
