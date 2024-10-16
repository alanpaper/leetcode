/// 3194. 最小元素和最大元素的最小平均值
/// 你有一个初始为空的浮点数数组 averages。另给你一个包含 n 个整数的数组 nums，其中 n 为偶数。
/// 你需要重复以下步骤 n / 2 次：
/// 从 nums 中移除 最小 的元素 minElement 和 最大 的元素 maxElement。
/// 将 (minElement + maxElement) / 2 加入到 averages 中。
/// 返回 averages 中的 最小 元素。

pub fn minimum_average(nums: Vec<i32>) -> f64 {
    let mut nums = nums.clone();
    nums.sort();
    let len = nums.len() / 2;
    let mut ans = (nums[0] + nums[nums.len() - 1]) as f64 / 2.0;
    for i in 0..len {
        let avg = (nums[i] + nums[nums.len() - 1 - i]) as f64 / 2.0;
        ans = ans.min(avg);
    }
    ans
}

#[test]
fn test_1() {
    assert_eq!(minimum_average(vec![7, 8, 3, 4, 15, 13, 4, 1]), 5.5)
}
#[test]
fn test_2() {
    assert_eq!(minimum_average(vec![1, 9, 8, 3, 10, 5]), 5.5)
}
#[test]
fn test_3() {
    assert_eq!(minimum_average(vec![1, 2, 3, 7, 8, 9]), 5.0)
}
