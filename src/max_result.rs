/// 1696. 跳跃游戏VI
///
/// 给你一个下标从 0 开始的整数数组 nums 和一个整数 k 。
// 一开始你在下标 0 处。每一步，你最多可以往前跳 k 步，但你不能跳出数组的边界。也就是说，你可以从下标 i 跳到 [i + 1， min(n - 1, i + k)] 包含 两个端点的任意位置。
// 你的目标是到达数组最后一个位置（下标为 n - 1 ），你的 得分 为经过的所有数字之和。
// 请你返回你能得到的 最大得分 。
use std::collections::VecDeque;

pub fn max_result(mut nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let k = k as usize;
    let mut q = VecDeque::new();
    q.push_back(0);
    for i in 1..n {
        // 1. 出
        if *q.front().unwrap() + k < i {
            q.pop_front();
        }

        // 2. 转移
        nums[i] += nums[*q.front().unwrap()];
        // 3. 入
        while !q.is_empty() && nums[i] >= nums[*q.back().unwrap()] {
            q.pop_back();
        }
        q.push_back(i);
    }
    nums[n - 1]
}

#[test]
fn test_1() {
    assert_eq!(max_result(vec![1, -1, -2, 4, -7, 3], 2), 7);
}

#[test]
fn test_2() {
    assert_eq!(max_result(vec![10, -5, -2, 4, 0, 3], 3), 17);
}
#[test]
fn test_3() {
    assert_eq!(max_result(vec![1, -5, -20, 4, -1, 3, -6, -3], 2), 0);
}

#[test]
fn test_4() {
    assert_eq!(
        max_result(vec![100, -100, -300, -300, -300, -100, 100], 4),
        0
    );
}

#[test]
fn test_5() {
    assert_eq!(max_result(vec![0, -1, -2, -3, 1], 2), -2);
}
