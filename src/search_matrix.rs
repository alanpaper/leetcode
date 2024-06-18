use std::cmp::Ordering;
/// 74. 搜索二维矩阵
/// 给你一个满足下述两条属性的 m x n 整数矩阵：
/// 每行中的整数从左到右按非严格递增顺序排列。
/// 每行的第一个整数大于前一行的最后一个整数。
/// 给你一个整数 target ，如果 target 在矩阵中，返回 true；否则，返回 false。
pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let mut top = 0;
    let mut bottom = matrix.len();
    if matrix[0][0] > target {
        return false;
    }
    if matrix[matrix.len() - 1][matrix[0].len() - 1] < target {
        return false;
    }
    while top <= bottom {
        let mid = (top + bottom) / 2;
        println!("{:?} -- mid", mid);
        match matrix[mid as usize][0].cmp(&target) {
            Ordering::Equal => {
                return true;
            }
            Ordering::Less => bottom = mid - 1,
            Ordering::Greater => top = mid + 1,
        }
    }
    false
}

#[test]
fn test_1() {
    let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
    let target = 3;
    assert_eq!(search_matrix(matrix, target), true);
}
// #[test]
// fn test_2() {
//     let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
//     let target = 13;
//     assert_eq!(search_matrix(matrix, target), false);
// }
// #[test]
// fn test_3() {
//     let matrix = vec![vec![1]];
//     let target = 0;
//     assert_eq!(search_matrix(matrix, target), false);
// }

// #[test]
// fn test_4() {
//     let matrix = vec![vec![1, 1]];
//     let target = 0;
//     assert_eq!(search_matrix(matrix, target), false);
// }
