/// 3180. 执行操作可获的最大总奖励I
/// 给你一个整数数组 rewardValues，长度为 n，代表奖励的值。
/// 最初，你的总奖励 x 为 0，所有下标都是 未标记 的。你可以执行以下操作 任意次 ：
/// 从区间 [0, n - 1] 中选择一个 未标记 的下标 i。
/// 如果 rewardValues[i] 大于 你当前的总奖励 x，
/// 则将 rewardValues[i] 加到 x 上（即 x = x + rewardValues[i]），并 标记 下标 i。
/// 以整数形式返回执行最优操作能够获得的 最大 总奖励。

pub fn max_total_reward(reward_values: Vec<i32>) -> i32 {
  0
}

#[test]
fn test_1() {
    assert_eq!(max_total_reward(vec![1, 1, 3, 3]), 4);
}

#[test]
fn test_2() {
    assert_eq!(max_total_reward(vec![1, 6, 4, 3, 2]), 11);
}
