/// 2769. 找出最大的可达成数字
/// 给你两个整数 num 和 t 。
/// 如果整数 x 可以在执行下述操作不超过 t 次的情况下变为与 num 相等，则称其为 可达成数字：
/// 每次操作将 x 的值增加或减少 1 ，同时可以选择将 num 的值增加或减少 1 。
/// 返回所有可达成数字中的最大值。可以证明至少存在一个可达成数字。

pub fn the_maximum_achievable_x(num: i32, t: i32) -> i32 {
    num + t + t
}

#[test]
fn test_1() {
    assert_eq!(the_maximum_achievable_x(4, 1), 6);
}
#[test]
fn test_2() {
    assert_eq!(the_maximum_achievable_x(3, 2), 7);
}
