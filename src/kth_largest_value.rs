use std::collections::HashMap;

/// 1738. 找出第K大的异或坐标值
/// 给你一个二维矩阵 matrix 和一个整数 k ，矩阵大小为 m x n 由非负整数组成。
/// 矩阵中坐标 (a, b) 的 目标值 可以通过对所有元素 matrix[i][j] 执行异或运算得到，
/// 其中 i 和 j 满足 0 <= i <= a < m 且 0 <= j <= b < n（下标从 0 开始计数）。
/// 请你找出 matrix 的所有坐标中第 k 大的目标值（k 的值从 1 开始计数)。

pub fn kth_largest_value(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
    let mut map = HashMap::new();
    let mut m = matrix.len();
    let mut n = matrix[0].len();
    for i in 0..m {
        for j in 0..n {}
    }
    0
}

#[test]
fn test_1() {
    assert_eq!(kth_largest_value(vec![vec![5, 2], vec![1, 6]], 1), 7);
}
#[test]
fn test_2() {
    assert_eq!(kth_largest_value(vec![vec![5, 2], vec![1, 6]], 2), 5);
}
#[test]
fn test_3() {
    assert_eq!(kth_largest_value(vec![vec![5, 2], vec![1, 6]], 3), 4);
}
#[test]
fn test_4() {
    assert_eq!(kth_largest_value(vec![vec![5, 2], vec![1, 6]], 4), 0);
}
