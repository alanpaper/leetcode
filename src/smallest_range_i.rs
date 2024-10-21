/// 908. 最小差值I
/// 给你一个整数数组 nums，和一个整数 k 。
/// 在一个操作中，您可以选择 0 <= i < nums.length 的任何索引 i 。
/// 将 nums[i] 改为 nums[i] + x ，其中 x 是一个范围为 [-k, k] 的任意整数。对于每个索引 i ，最多 只能 应用 一次 此操作。
/// nums 的 分数 是 nums 中最大和最小元素的差值。
/// 在对  nums 中的每个索引最多应用一次上述操作后，返回 nums 的最低 分数 。

pub fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
    let mut min = nums[0];
    let mut max = nums[0];
    for n in nums {
        max = max.max(n);
        min = min.min(n);
    }
    if max - min > 2 * k {
        return max - min - 2 * k;
    }
    return 0;
}

#[test]
fn test_1() {
    assert_eq!(smallest_range_i(vec![1], 0), 0);
}
#[test]
fn test_2() {
    assert_eq!(smallest_range_i(vec![0, 10], 2), 6);
}
#[test]
fn test_3() {
    assert_eq!(smallest_range_i(vec![1, 3, 6], 3), 0);
}

/// 908. 最小差值II
/// 给你一个整数数组 nums，和一个整数 k 。
/// 对于每个下标 i（0 <= i < nums.length），将 nums[i] 变成  nums[i] + k 或 nums[i] - k 。
/// nums 的 分数 是 nums 中最大元素和最小元素的差值。
/// 在更改每个下标对应的值之后，返回 nums 的最小 分数 。
///
pub fn smallest_range_ii(nums: Vec<i32>, k: i32) -> i32 {
    0
}
#[test]
fn test_i1() {
    assert_eq!(smallest_range_ii(vec![1], 0), 0);
}
#[test]
fn test_i2() {
    assert_eq!(smallest_range_ii(vec![0, 10], 2), 6);
}
#[test]
fn test_i3() {
    assert_eq!(smallest_range_ii(vec![1, 3, 6], 3), 3);
}
