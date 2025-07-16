use std::collections::HashMap;

/// 2461. 长度为 K 子数组中的最大和

/// 给你一个整数数组 nums 和一个整数 k 。请你从 nums 中满足下述条件的全部子数组中找出最大子数组和：

/// 子数组的长度是 k，且
/// 子数组中的所有元素 各不相同 。
/// 返回满足题面要求的最大子数组和。如果不存在子数组满足这些条件，返回 0 。

/// 子数组 是数组中一段连续非空的元素序列。
///

pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
    let mut ans: i64 = 0;
    let mut total: i64 = 0;
    let mut screen_map = HashMap::new();
    let mut left_i: usize = 0;
    let mut right_i: usize = 0;

    for (index, &num) in nums.iter().enumerate() {
        match screen_map.get(&(num as i64)) {
            Some(&n) => {
                while n as i64 >= left_i as i64 {
                    total -= nums[left_i] as i64;
                    left_i += 1;
                }
            }
            None => {}
        }
        screen_map.insert(num as i64, index);
        total += num as i64;
        if right_i - left_i + 1 == k as usize {
            if ans < total {
                ans = total;
            }
            total -= nums[left_i] as i64;
            left_i += 1;
        }
        right_i += 1;
    }
    ans
}

#[test]
fn test_max() {
    let nums = vec![1, 5, 4, 2, 9, 9, 9];
    let k = 3;
    assert_eq!(maximum_subarray_sum(nums, k), 15);
}
#[test]
fn test_max_two() {
    let nums = vec![1, 2, 3, 4];
    let k = 4;
    assert_eq!(maximum_subarray_sum(nums, k), 10);
}
#[test]
fn test_max_three() {
    let nums = vec![9, 9, 9, 1, 2, 3];
    let k = 3;
    assert_eq!(maximum_subarray_sum(nums, k), 12);
}
