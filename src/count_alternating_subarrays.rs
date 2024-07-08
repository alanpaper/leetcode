/// 3101. 交替子数组计数
/// 给你一个二进制数组nums 。
// 如果一个子数组中 不存在 两个 相邻 元素的值 相同 的情况，我们称这样的子数组为 交替子数组 。
// 返回数组 nums 中交替子数组的数量。

pub fn count_alternating_subarrays(nums: Vec<i32>) -> i64 {
    let mut total = nums.len();
    let mut pre = nums[0];
    let mut sub_total = 1;
    for i in 1..nums.len() {
        if pre + nums[i] == 1 {
            sub_total += 1;
        } else if sub_total > 1 {
            total += sub_total * (sub_total - 1) / 2;
            sub_total = 1;
        }
        pre = nums[i];
    }
    if sub_total > 1 {
        total += sub_total * (sub_total - 1) / 2;
    }
    total as i64
}

#[test]
fn test_1() {
    assert_eq!(count_alternating_subarrays(vec![0, 1, 1, 1]), 5)
}

#[test]
fn test_2() {
    assert_eq!(
        count_alternating_subarrays(vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1]),
        76
    )
}
