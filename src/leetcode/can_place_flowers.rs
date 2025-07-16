/// 605. 种花问题
/// 假设有一个很长的花坛，一部分地块种植了花，另一部分却没有。可是，花不能种植在相邻的地块上，它们会争夺水源，两者都会死去。
/// 给你一个整数数组 flowerbed 表示花坛，由若干 0 和 1 组成，其中 0 表示没种植花，1 表示种植了花。另有一个数 n ，
/// 能否在不打破种植规则的情况下种入 n 朵花？能则返回 true ，不能则返回 false 。
pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
    let mut list = flowerbed.clone();
    let mut pre = -1;
    list.push(-1);
    let mut ans: i32 = 0;
    for i in 0..(list.len() - 1) {
        if list[i] == 0 && pre <= 0 && list[i + 1] <= 0 {
            ans += 1;
            list[i] = 1;
        }
        pre = list[i];
    }
    ans >= n
}

#[test]
fn can_place_flowers_test() {
    let larger_len = can_place_flowers(vec![0, 0, 1, 0, 1], 1);
    assert_eq!(larger_len, true);
}
#[test]
fn can_place_flowers_test_1() {
    let larger_len = can_place_flowers(vec![1, 0, 0, 0, 1, 0, 0], 2);
    assert_eq!(larger_len, true);
}
