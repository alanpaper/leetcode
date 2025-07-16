/// 2742. 给墙壁刷油漆
/// 给你两个长度为 n 下标从 0 开始的整数数组 cost 和 time ，分别表示给 n 堵不同的墙刷油漆需要的开销和时间。你有两名油漆匠：
// 一位需要 付费 的油漆匠，刷第 i 堵墙需要花费 time[i] 单位的时间，开销为 cost[i] 单位的钱。
// 一位 免费 的油漆匠，刷 任意 一堵墙的时间为 1 单位，开销为 0 。但是必须在付费油漆匠 工作 时，免费油漆匠才会工作。
// 请你返回刷完 n 堵墙最少开销为多少。

pub fn paint_walls(cost: Vec<i32>, time: Vec<i32>) -> i32 {
    0
}

// #[test]
// fn test_1() {
//     let cost = vec![1, 2, 3, 2];
//     let time = vec![1, 2, 3, 2];
//     assert_eq!(paint_walls(cost, time), 3);
// }

// #[test]
// fn test_2() {
//     let cost = vec![2, 3, 4, 2];
//     let time = vec![1, 1, 1, 1];
//     assert_eq!(paint_walls(cost, time), 4);
// }

// #[test]
// fn test_3() {
//     let cost = vec![49, 35, 32, 20, 30, 12, 42];
//     let time = vec![1, 1, 2, 2, 1, 1, 2];
//     assert_eq!(paint_walls(cost, time), 62);
// }

// #[test]
// fn test_4() {
//     let cost = vec![26, 53, 10, 24, 25, 20, 63, 51];
//     let time = vec![1, 1, 1, 1, 2, 2, 2, 1];
//     assert_eq!(paint_walls(cost, time), 55);
// }

#[test]
fn test_5() {
    let cost = vec![76, 25, 96, 46, 85, 19, 29, 88, 2, 5];
    let time = vec![1, 2, 1, 3, 1, 3, 3, 3, 2, 1];
    assert_eq!(paint_walls(cost, time), 46);
}

// #[test]
// fn test_6() {
//     let cost = vec![7, 15, 38, 35, 61, 90, 34, 29, 68, 35];
//     let time = vec![1, 1, 3, 3, 2, 1, 3, 1, 2, 3];
//     assert_eq!(paint_walls(cost, time), 76);
// }
