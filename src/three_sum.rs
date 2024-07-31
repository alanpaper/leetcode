use std::collections::HashSet;

/// 15. 三数之和
/// 给你一个整数数组 nums ，判断是否存在三元组 [nums[i], nums[j], nums[k]] 满足 i != j、i != k 且 j != k ，
/// 同时还满足 nums[i] + nums[j] + nums[k] == 0 。请你返回所有和为 0 且不重复的三元组。
/// 注意：答案中不可以包含重复的三元组。
///
pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ans = vec![];
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            let mut num_k = None;
            for k in (j + 1)..nums.len() {
                if let Some(n_k) = num_k {
                    if n_k == nums[k] {
                        continue;
                    }
                }
                if i != j && i != k && j != k && nums[i] + nums[j] + nums[k] == 0 {
                    num_k = Some(nums[k]);
                    ans.push(vec![nums[i], nums[j], nums[k]]);
                }
            }
        }
    }
    ans
}

#[test]
fn test_1() {
    let nums = vec![-1, 0, 1, 2, -1, -4];
    assert_eq!(three_sum(nums), vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
}

#[test]
fn test_2() {
    let nums = vec![0, 0, 0];
    assert_eq!(three_sum(nums), vec![vec![0, 0, 0]]);
}
