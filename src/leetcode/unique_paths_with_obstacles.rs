pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    let m = obstacle_grid.len();
    let n = obstacle_grid[0].len();

    let mut f = vec![0; n];
    f[0] = 1;
    for i in 1..m {
        for j in 1..n {
            if obstacle_grid[i][j] == 1 {
                f[j] = 0;
            } else if j > 0 && obstacle_grid[i][j - 1] == 0 {
                f[j] += f[j - 1];
            }
        }
    }

    f[n - 1]
}

#[test]
fn test_1() {
    assert_eq!(
        unique_paths_with_obstacles(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]),
        2
    );
}
