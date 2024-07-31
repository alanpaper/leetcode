use std::collections::HashSet;
/// 3111. 覆盖所有点的最少矩阵数目
/// 给你一个二维整数数组 point ，其中 points[i] = [xi, yi] 表示二维平面内的一个点。同时给你一个整数 w 。你需要用矩形 覆盖所有 点。
/// 每个矩形的左下角在某个点 (x1, 0) 处，且右上角在某个点 (x2, y2) 处，其中 x1 <= x2 且 y2 >= 0 ，同时对于每个矩形都 必须 满足 x2 - x1 <= w 。
/// 如果一个点在矩形内或者在边上，我们说这个点被矩形覆盖了。
/// 请你在确保每个点都 至少 被一个矩形覆盖的前提下，最少 需要多少个矩形。
/// 注意：一个点可以被多个矩形覆盖。
///
pub fn min_rectangles_to_cover_points(points: Vec<Vec<i32>>, w: i32) -> i32 {
    let mut point_x_set = HashSet::new();
    let mut min = points[0][0];
    let mut max = points[0][0];
    for p in points {
        point_x_set.insert(p[0]);
        min = min.min(p[0]);
        max = max.max(p[0]);
    }
    max += 1;
    let mut ans = 1;
    let mut pre = min;
    for x in min..max {
        if point_x_set.contains(&x) && x > pre + w {
            pre = x;
            ans += 1;
        }
    }
    ans
}

#[test]
fn test_1() {
    let points = vec![
        vec![2, 1],
        vec![1, 0],
        vec![1, 4],
        vec![1, 8],
        vec![3, 5],
        vec![4, 6],
    ];
    assert_eq!(min_rectangles_to_cover_points(points, 1), 2);
}
#[test]
fn test_2() {
    let points = vec![
        vec![0, 0],
        vec![1, 1],
        vec![2, 2],
        vec![3, 3],
        vec![4, 4],
        vec![5, 5],
        vec![6, 6],
    ];
    assert_eq!(min_rectangles_to_cover_points(points, 2), 3);
}

#[test]
fn test_3() {
    let points = vec![vec![2, 3], vec![1, 2]];
    assert_eq!(min_rectangles_to_cover_points(points, 0), 2);
}
