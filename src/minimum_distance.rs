/// 3102. 最小曼哈顿距离
///
/// 给你一个下标从 0 开始的数组 points ，它表示二维平面上一些点的整数坐标，其中 points[i] = [xi, yi] 。
// 两点之间的距离定义为它们的曼哈顿距离。
// 请你恰好移除一个点，返回移除后任意两点之间的 最大 距离可能的 最小 值。
pub fn minimum_distance(points: Vec<Vec<i32>>) -> i32 {
    let mut points_map = points.clone();

    0
}

#[test]
fn test_1() {
    let points = vec![vec![3, 10], vec![5, 15], vec![10, 2], vec![4, 4]];
    assert_eq!(minimum_distance(points), 12);
}
#[test]
fn test_2() {
    let points = vec![vec![1, 1], vec![1, 1], vec![1, 1]];
    assert_eq!(minimum_distance(points), 0);
}
