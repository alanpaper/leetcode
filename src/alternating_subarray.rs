// 2765. Longest Continuous Increasing Subsequence
// https://leetcode-cn.com/problems/longest-continuous-increasing-subsequence/

// Your code goes here
impl Solution {
    pub fn longest_continuous_increasing_subsequence(nums: Vec<i32>) -> i32 {
        let mut max_len = 1;
        let mut curr_len = 1;

        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                curr_len += 1;
                max_len = max_len.max(curr_len);
            } else {
                curr_len = 1;
            }
        }

        max_len
    }
}


