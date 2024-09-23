/// 1014. 最佳观光组合
/// 给你一个正整数数组 values，其中 values[i] 表示第 i 个观光景点的评分，并且两个景点 i 和 j 之间的 距离 为 j - i。
/// 一对景点（i < j）组成的观光组合的得分为 values[i] + values[j] + i - j ，也就是景点的评分之和 减去 它们两者之间的距离。
/// 返回一对观光景点能取得的最高分。
pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut max = 0;
    for i in 0..values.len() {
        ans = ans.max(max + values[i] - (i as i32));
        max = max.max(values[i] + (i as i32))
    }
    ans
}

#[test]
fn test_1() {
    assert_eq!(max_score_sightseeing_pair(vec![8, 1, 5, 2, 6]), 11);
}

#[test]
fn test_2() {
    assert_eq!(max_score_sightseeing_pair(vec![1, 2]), 2);
}
