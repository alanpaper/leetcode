/// 200. 岛屿数量
///
///
pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    let mut island = vec![];

    let m = grid.len();
    let n = grid[0].len();

    for i in 0..m {
        for j in 0..n {
            if let Some(land) = grid.get(i).unwrap().get(j) {
                if land == &'1' {
                    island.push((i, j));
                }
            }
        }
    }

    for point in island {}

    0
}

fn dfs(point: (usize, usize), lands: Vec<(usize, usize)>) {
    let mut points = vec![];

    for p in lands {
        if (p.0 - point.0) as i32 == 1i32.abs() || (p.1 - point.1) as i32 == 1i32.abs() {
            points.push(p);
        }
    }
}

#[test]
fn test_1() {
    let grid = vec![
        vec!['1', '1', '1', '1', '0'],
        vec!['1', '1', '0', '1', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '0', '0', '0'],
    ];
    assert_eq!(num_islands(grid), 1);
}

#[test]
fn test_2() {
    let grid = vec![
        vec!['1', '1', '0', '0', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '1', '0', '0'],
        vec!['0', '0', '0', '1', '1'],
    ];
    assert_eq!(num_islands(grid), 3);
}
