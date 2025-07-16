/// 2938. 区分黑球和白球
/// 桌子上有 n 个球，每个球的颜色不是黑色，就是白色。
/// 给你一个长度为 n 、下标从 0 开始的二进制字符串 s，其中 1 和 0 分别代表黑色和白色的球。
/// 在每一步中，你可以选择两个相邻的球并交换它们。
/// 返回「将所有黑色球都移到右侧，所有白色球都移到左侧所需的 最小步数」。
///
pub fn minimum_steps(s: String) -> i64 {
    let mut ans = 0;

    let mut step_1 = 0;

    for i in s.chars() {
        if i == '1' {
            step_1 += 1;
        } else {
            ans += step_1;
        }
    }
    ans
}

// 1010
// 0110
// 0101
// 0011

#[test]
fn test_1() {
    assert_eq!(minimum_steps(String::from("1010")), 3)
}

#[test]
fn test_2() {
    assert_eq!(minimum_steps(String::from("100")), 2)
}

#[test]
fn test_3() {
    assert_eq!(minimum_steps(String::from("0111")), 0)
}
