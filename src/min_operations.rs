/// 3191. 使二进制数组全部等于1的最少操作次数I
/// 给你一个二进制数组 nums 。
/// 你可以对数组执行以下操作 任意 次（也可以 0 次）：
/// 选择数组中 任意连续 3 个元素，并将它们 全部反转 。
/// 反转 一个元素指的是将它的值从0变1 ，或者从1变0。
/// 请你返回将nums中所有元素变为1的最少操作次数。如果无法全部变成1，返回 -1。
pub fn min_operations(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut nums = nums.clone();
    for i in 0..(nums.len() - 2) {
        if nums[i] == 0 {
            nums[i + 1] ^= 1;
            nums[i + 2] ^= 1;
            ans += 1;
        }
    }
    if nums[nums.len() - 1] == 0 || nums[nums.len() - 2] == 0 {
        return -1;
    }
    ans
}

#[test]
fn test_1() {
    assert_eq!(min_operations(vec![0, 1, 1, 1, 0, 0]), 3);
}
#[test]
fn test_2() {
    assert_eq!(min_operations(vec![0, 1, 1, 1]), -1);
}
