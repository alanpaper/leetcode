/// 2808. 使循环数所有元素相等的最少秒数
/// 给你一个下标从 0 开始长度为 n 的数组 nums 。
/// 每一秒，你可以对数组执行以下操作：
/// 对于范围在 [0, n - 1] 内的每一个下标 i ，将 nums[i] 替换成 nums[i]，nums[(i - 1 + n) % n] 或者 nums[(i + 1) % n] 三者之一。
/// 注意，所有元素会被同时替换。
/// 请你返回将数组 nums 中所有元素变成相等元素所需要的 最少秒数。
pub fn minimum_seconds(nums: Vec<i32>) -> i32 {
    0
}

#[test]
fn test_1() {
    assert_eq!(minimum_seconds(vec![1, 2, 1, 2]), 1);
}

#[test]
fn test_2() {
    assert_eq!(minimum_seconds(vec![2, 1, 3, 3, 2]), 2);
}

#[test]
fn test_3() {
    assert_eq!(minimum_seconds(vec![5, 5, 5, 5]), 0);
}
