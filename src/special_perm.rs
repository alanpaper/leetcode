/// 2741. 特别的排列
/// 给你一个下标从 0 开始的整数数组 nums ，它包含 n 个 互不相同 的正整数。如果 nums 的一个排列满足以下条件，我们称它是一个特别的排列：
/// 对于 0 <= i < n - 1 的下标 i ，要么 nums[i] % nums[i+1] == 0 ，要么 nums[i+1] % nums[i] == 0 。
/// 请你返回特别排列的总数目，由于答案可能很大，请将它对 109 + 7 取余 后返回。
///

pub fn special_perm(nums: Vec<i32>) -> i32 {}

#[test]
fn test_1() {
    assert_eq!(special_perm(vec![2, 3, 6]), 2);
}

#[test]
fn test_2() {
    assert_eq!(special_perm(vec![1, 4, 3]), 2);
}
