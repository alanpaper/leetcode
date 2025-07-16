/// 377. 组合总和 IV
/// 给你一个由 不同 整数组成的数组nums，和一个目标整数target。请你从 nums 中找出并返回总和为 target 的元素组合的个数。
/// 题目数据保证答案符合32位整数范围。
pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
    let mut dp = vec![0; (target + 1) as usize];
    dp[0] = 1;
    for i in 0..4 {
        for n in &nums {
            let index = i + 1;
            if *n <= index {
                dp[index as usize] += dp[(index - *n) as usize]
            }
        }
    }
    println!("{:?}", dp);
    dp[0]
}

#[test]
fn test_1() {
    assert_eq!(combination_sum4(vec![1, 2, 3], 4), 7);
}

#[test]
fn test_2() {
    assert_eq!(combination_sum4(vec![9], 3), 0);
}
