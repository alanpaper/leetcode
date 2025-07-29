/// 64. 64. 最小路径和
/// 给定一个包含非负整数的 m x n 网格 grid ，请找出一条从左上角到右下角的路径，使得路径上的数字总和为最小。
/// 说明：每次只能向下或者向右移动一步。

pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {

    let m = grid.len();
    let n = grid[0].len();

    let mut f = vec![vec![0; m]; n];
    f[0][0] = grid[0][0];

    for i in 1..m  {
        for j in 1..n {
           
        }
    }



    0

    
}

#[test]
fn min_path_sum_test() {
    assert_eq!(
        min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]),
        7
    );
}

// 62. 不同路径
pub fn unique_paths(m: i32, n: i32) -> i32 {
    0
}

#[test]
fn unique_paths_test() {
    assert_eq!(unique_paths(3, 7), 28);
}
