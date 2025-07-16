/// 807. 保持城市天际线
pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut row = vec![0; m];
    let mut col = vec![0; n];

    for i in 0..m {
        for j in 0..n {
            row[i] = row[i].max(grid[i][j]);
            col[j] = col[j].max(grid[i][j]);
        }
    }

    let mut total = 0;
    for i in 0..m {
        for j in 0..n {
            total += col[j].min(row[i]) - grid[i][j];
        }
    }

    total
}

#[test]
fn test_1() {
    let grid = vec![
        vec![3, 0, 8, 4],
        vec![2, 4, 5, 7],
        vec![9, 2, 6, 3],
        vec![0, 3, 1, 0],
    ];
    assert_eq!(max_increase_keeping_skyline(grid), 35);
}
