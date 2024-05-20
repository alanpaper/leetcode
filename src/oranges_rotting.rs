use std::collections::VecDeque;
/// 994. 腐烂的橘子
pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
    // 好橘子到坏橘子的最短路径
    let mut good_oranges: Vec<(usize, usize)> = vec![];

    let m = grid.len();
    let n = grid[0].len();

    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 1 {
                good_oranges.push((i, j));
            }
        }
    }

    let mut ans: Vec<(usize, usize, i32)> = vec![];
    for orange in &good_oranges {
        let distance = bfs(orange, &grid);

        println!("{:?}=== distance", distance);

        if distance == -1 {
            return -1;
        }
        ans.push((orange.0, orange.1, distance));
    }

    let mut max_distance = 0;
    for num in ans {
        max_distance = max_distance.max(num.2)
    }
    max_distance
}

fn bfs(orange: &(usize, usize), grid: &Vec<Vec<i32>>) -> i32 {
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut distance = 0;
    let m = orange.0;
    let n = orange.1;
    queue.push_back((m, n));

    println!("{:?}==queue", queue);

    while !queue.is_empty() {
        println!("{:?}==queue=={:?}", queue, distance);
        if let Some(orange_distance) = queue.pop_front() {
            if is_bad_leaf(grid, (orange_distance.0, orange_distance.1)) {
                distance += 1;
                return distance;
            }

            if is_leaf(grid, (orange_distance.0, orange_distance.1)) {
                return -1;
            }

            // 上
            if orange_distance.0 - 1 > 0 && grid[orange_distance.0 - 1][orange_distance.1] == 1 {
                queue.push_back((orange_distance.0 - 1, orange_distance.1));
            }
            // 下
            if orange_distance.0 + 1 < grid.len() - 1
                && grid[orange_distance.0 - 1][orange_distance.1] == 1
            {
                queue.push_back((orange_distance.0 + 1, orange_distance.1));
            }

            // 左
            if orange_distance.1 - 1 > 0 && grid[orange_distance.0][orange_distance.1 - 1] == 1 {
                queue.push_back((orange_distance.0, orange_distance.1 - 1));
            }

            // 右
            if orange_distance.1 + 1 < grid[0].len() - 1
                && grid[orange_distance.0][orange_distance.1 - 1] == 1
            {
                queue.push_back((orange_distance.0, orange_distance.1 - 1));
            }

            distance += 1;
        }
    }

    distance
}

// 判断该节点是不是坏的叶子节点
fn is_bad_leaf(grid: &Vec<Vec<i32>>, orange: (usize, usize)) -> bool {
    if orange.0 != 0 && orange.0 - 1 >= 0 && grid[orange.0 - 1][orange.1] == 2 {
        return true;
    }
    if orange.0 + 1 < grid.len() - 1 && grid[orange.0 + 1][orange.1] == 2 {
        return true;
    }
    if orange.1 != 0 && orange.1 - 1 >= 0 && grid[orange.0][orange.1 - 1] == 2 {
        return true;
    }
    if orange.1 + 1 < grid[0].len() - 1 && grid[orange.0][orange.1 + 1] == 2 {
        return true;
    }
    false
}

// 判断该节点是不是叶子节点
fn is_leaf(grid: &Vec<Vec<i32>>, orange: (usize, usize)) -> bool {
    let mut is_top = false;
    let mut is_bottom = false;
    let mut is_left = false;
    let mut is_right = false;
    if orange.0 != 0 && orange.0 - 1 >= 0 && grid[orange.0 - 1][orange.1] == 0 {
        is_top = true;
    }
    if orange.0 + 1 < grid.len() - 1 && grid[orange.0 + 1][orange.1] == 0 {
        is_bottom = true;
    }
    if orange.1 != 0 && orange.1 - 1 >= 0 && grid[orange.0][orange.1 - 1] == 0 {
        is_left = true;
    }
    if orange.1 + 1 < grid[0].len() - 1 && grid[orange.0][orange.1 + 1] == 0 {
        is_right = true;
    }

    if orange.0 == 0 && orange.1 == 0 {
        return is_bottom && is_right;
    }
    if orange.0 == 0 && orange.1 == grid[0].len() - 1 {
        return is_bottom && is_left;
    }
    if orange.0 == 0 {
        return is_bottom && is_left && is_right;
    }

    if orange.0 == grid.len() - 1 && orange.1 == grid[0].len() - 1 {
        return is_bottom && is_right;
    }
    if orange.0 == 0 && orange.1 == grid[0].len() - 1 {
        return is_bottom && is_left;
    }
    if orange.0 == 0 {
        return is_bottom && is_left && is_right;
    }

    if orange.1 == 0 && orange.1 == 0 {
        return is_bottom && is_right;
    }
    if orange.1 == 0 && orange.1 == grid[0].len() - 1 {
        return is_bottom && is_left;
    }
    if orange.1 == 0 {
        return is_bottom && is_left && is_right;
    }

    is_top && is_bottom && is_left && is_right
}

#[test]
fn test_1() {
    // 2 1 1
    // 1 1 0
    // 0 1 1

    assert_eq!(
        oranges_rotting(vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]]),
        4
    );
}
#[test]
fn test_2() {
    assert_eq!(
        oranges_rotting(vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]]),
        -1
    );
}



