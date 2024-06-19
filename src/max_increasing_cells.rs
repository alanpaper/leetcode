use std::collections::HashMap;

/// 2713. 矩阵中严格递增的单元格数
/// 给你一个下标从 1 开始、大小为 m x n 的整数矩阵 mat，你可以选择任一单元格作为 起始单元格。
/// 从起始单元格出发，你可以移动到 同一行或同一列 中的任何其他单元格，但前提是目标单元格的值 严格大于 当前单元格的值。
/// 你可以多次重复这一过程，从一个单元格移动到另一个单元格，直到无法再进行任何移动。
/// 请你找出从某个单元开始访问矩阵所能访问的 单元格的最大数量 。
/// 返回一个表示可访问单元格最大数量的整数。
pub fn max_increasing_cells(mat: Vec<Vec<i32>>) -> i32 {
    let m = mat.len();
    let n = mat[0].len();
    let mut mp: HashMap<i32, Vec<(usize, usize)>> = HashMap::new();
    let mut row = vec![0; m];
    let mut col = vec![0; n];
    for i in 0..m {
        for j in 0..n {
            mp.entry(mat[i][j]).or_insert(Vec::new()).push((i, j));
        }
    }
    let mut sorted_mp: Vec<_> = mp.iter().collect();
    sorted_mp.sort_by(|a, b| a.0.cmp(b.0));
    println!("sorted_mp = {:#?}", sorted_mp);

    for (_, pos) in sorted_mp {
        let res: Vec<_> = pos.iter().map(|&(i, j)| row[i].max(col[j]) + 1).collect(); // 存放相同数值的答案，便于后续更新 row 和 col
        for (&(i, j), &d) in pos.iter().zip(res.iter()) {
            row[i] = row[i].max(d);
            col[j] = col[j].max(d);
        }
    }
    *row.iter().max().unwrap()
}

// 1 - 3 - 4 - 8 - 10
//     - 3 - 6 - 7 - 10
// 1 - 5 - 7 - 10
//     - 5 - 8 - 10
//  3 1 6
// -9 5 7
//  4 8 10

#[test]
fn test_1() {
    let mat = vec![vec![3, 1], vec![3, 4]];
    assert_eq!(max_increasing_cells(mat), 2);
}
#[test]
fn test_2() {
    let mat = vec![vec![1, 1], vec![1, 1]];
    assert_eq!(max_increasing_cells(mat), 1);
}

#[test]
fn test_3() {
    let mat = vec![vec![3, 1, 6], vec![-9, 5, 7]];
    assert_eq!(max_increasing_cells(mat), 4);
}
