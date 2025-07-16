/// 3033. 修改矩阵
// 给你一个下标从 0 开始、大小为 m x n 的整数矩阵 matrix ，新建一个下标从 0 开始、名为 answer 的矩阵。
/// 使 answer 与 matrix 相等，接着将其中每个值为 -1 的元素替换为所在列的 最大 元素。
// 返回矩阵 answer 。
pub fn modified_matrix(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let l = matrix[0].len();
    let mut ans = matrix.clone();
    for i in 0..l {
        let mut ma = -1;
        for j in 0..matrix.len() {
            ma = ma.max(matrix[j][i]);
        }
        for j in 0..matrix.len() {
            if matrix[j][i] == -1 {
                ans[j][i] = ma;
            }
        }
    }
    ans
}

#[test]
fn test_1() {
    let matrix = vec![vec![], vec![]];
    assert_eq!(modified_matrix(matrix.clone()), matrix);
}
