/// 3259. 超级饮料的最大强化能量
/// 来自未来的体育科学家给你两个整数数组 energyDrinkA 和 energyDrinkB，数组长度都等于 n。
/// 这两个数组分别代表 A、B 两种不同能量饮料每小时所能提供的强化能量。
/// 你需要每小时饮用一种能量饮料来 最大化 你的总强化能量。然而，如果从一种能量饮料切换到另一种，
/// 你需要等待一小时来梳理身体的能量体系（在那个小时里你将不会获得任何强化能量）。
/// 返回在接下来的 n 小时内你能获得的 最大 总强化能量。
/// 注意 你可以选择从饮用任意一种能量饮料开始。

pub fn max_energy_boost(energy_drink_a: Vec<i32>, energy_drink_b: Vec<i32>) -> i64 {
    let mut a_total = 0;
    let mut b_total = 0;
    for i in 0..energy_drink_a.len() {
        a_total += energy_drink_a[i] as i64;
        b_total += energy_drink_b[i] as i64;
    }

    a_total
}

#[test]
fn test() {
    assert_eq!(max_energy_boost(vec![1, 3, 1], vec![3, 1, 1]), 5);
}
#[test]
fn test_1() {
    assert_eq!(max_energy_boost(vec![4, 1, 1], vec![1, 1, 3]), 7);
}
