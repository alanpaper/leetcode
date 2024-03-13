/// 2369. 检查数组是否存在有效划分
pub fn valid_partition(nums: Vec<i32>) -> bool {
    let n = nums.len();
    let mut dp = vec![false; n + 1];
    dp[0] = true;
    for i in 1..(n + 1) {
        if i >= 2 {
            dp[i] = dp[i - 2] && nums[i - 2] == nums[i - 1];
        }
        if i >= 3 {
            dp[i] = dp[i]
                || (dp[i - 3]
                    && ((nums[i - 3] == nums[i - 2] && nums[i - 2] == nums[i - 1])
                        || (nums[i - 3] + 1 == nums[i - 2] && nums[i - 2] + 1 == nums[i - 1])));
        }
    }
    dp[n]
}

#[test]
fn test_valid_partition() {
    assert_eq!(valid_partition(vec![4, 4, 4, 5, 6]), true);
}
#[test]
fn test1_valid_partition() {
    assert_eq!(valid_partition(vec![1, 1, 1, 2]), false);
}
