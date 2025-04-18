/// 1184. 公交站间的距离
/// 环形公交路线上有 n 个站，按次序从 0 到 n - 1 进行编号。
/// 我们已知每一对相邻公交站之间的距离，distance[i] 表示编号为 i 的车站和编号为 (i + 1) % n 的车站之间的距离。
/// 环线上的公交车都可以按顺时针和逆时针的方向行驶。
/// 返回乘客从出发点 start 到目的地 destination 之间的最短距离。
pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {}

#[test]
fn test_1() {
    assert_eq!(distance_between_bus_stops(vec![1, 2, 3, 4], 0, 0), 0);
}
#[test]
fn test_2() {
    assert_eq!(distance_between_bus_stops(vec![1, 2, 3, 4], 2, 0), 3);
}

#[test]
fn test_3() {
    assert_eq!(distance_between_bus_stops(vec![1, 2, 3, 4], 0, 3), 4);
}
