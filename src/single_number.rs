/// 136. 只出现一次的数字
///
pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut ans = nums[0];
    for i in 1..nums.len() {
        ans = ans ^ nums[i];
    }
    ans
}

#[test]
fn test_1() {
    assert_eq!(single_number(vec![2, 2, 1]), 1)
}

/// 137. 只出现一次的数字II
/// 给你一个整数数组 nums ，除某个元素仅出现 一次 外，其余每个元素都恰出现 三次 。请你找出并返回那个只出现了一次的元素。
// 你必须设计并实现线性时间复杂度的算法且使用常数级空间来解决此问题。
pub fn single_number_v2(nums: Vec<i32>) -> i32 {
    0
}

#[test]
fn test_2() {
    assert_eq!(single_number(vec![2, 2, 3, 2]), 3)
}
