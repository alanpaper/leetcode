/// 2740. 找出分区值
///
// 给你一个 正 整数数组 nums 。
// 将 nums 分成两个数组：nums1 和 nums2 ，并满足下述条件：
// 数组 nums 中的每个元素都属于数组 nums1 或数组 nums2 。
// 两个数组都 非空 。
// 分区值 最小 。
// 分区值的计算方法是 |max(nums1) - min(nums2)| 。
// 其中，max(nums1) 表示数组 nums1 中的最大元素，min(nums2) 表示数组 nums2 中的最小元素。
// 返回表示分区值的整数。
pub fn find_value_of_partition(nums: Vec<i32>) -> i32 {
    let mut nums = nums.clone();
    nums.sort();
    let mut ans = *nums.last().unwrap();
    for i in 0..(nums.len() - 1) {
        ans = (nums[i] - nums[i + 1]).abs().min(ans);
    }
    ans
}

#[test]
fn test_1() {
    assert_eq!(find_value_of_partition(vec![1, 3, 2, 4]), 1);
}

#[test]
fn test_2() {
    assert_eq!(find_value_of_partition(vec![100, 1, 10]), 9);
}
