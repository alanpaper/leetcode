pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut dp: Vec<Vec<i32>> = vec![vec![grid[0][0]; n]; m];
    for y in 0..m {
        for x in 0..n {
            if y > 0 && x > 0 {
                dp[y][x] = dp[y - 1][x].min(dp[y][x - 1]) + grid[y][x]
            } else if x == 0 && y > 0 {
                dp[y][0] = grid[y][0] + dp[y - 1][0]
            } else if y == 0 && x > 0 {
                dp[0][x] = grid[0][x] + dp[0][x - 1]
            }
        }
    }
    dp[m - 1][n - 1]
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
