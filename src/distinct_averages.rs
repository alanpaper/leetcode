use std::collections::{HashSet, VecDeque};

/// 2465. 不同的平均值数目
/// 给你一个下标从 0 开始长度为 偶数 的整数数组 nums 。
/// 只要 nums 不是 空数组，你就重复执行以下步骤：
/// 找到 nums 中的最小值，并删除它。
/// 找到 nums 中的最大值，并删除它。
/// 计算删除两数的平均值。
/// 两数 a 和 b 的 平均值 为 (a + b) / 2 。
/// 比方说，2 和 3 的平均值是 (2 + 3) / 2 = 2.5 。
/// 返回上述过程能得到的 不同 平均值的数目。
/// 注意 ，如果最小值或者最大值有重复元素，可以删除任意一个。
///
pub fn distinct_averages(nums: Vec<i32>) -> i32 {
    let mut nums = nums.clone();
    nums.sort();
    let mut nums = VecDeque::from(nums);
    let mut ans = HashSet::new();
    while !nums.is_empty() {
        let one = nums.pop_front().unwrap();
        let last = nums.pop_back().unwrap();
        let total = one + last;
        ans.insert(format!("{}", (total as f32) / 2.0));
    }
    ans.len() as i32
}

#[test]
fn test_1() {
    assert_eq!(distinct_averages(vec![4, 1, 4, 0, 3, 5]), 2);
}

#[test]
fn test_2() {
    assert_eq!(distinct_averages(vec![1, 100]), 1);
}

#[test]
fn test_3() {
    assert_eq!(distinct_averages(vec![9, 5, 7, 8, 7, 9, 8, 2, 0, 7]), 5);
}
